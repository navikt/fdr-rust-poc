{
  description = "A Nix-flake-based Rust development environment";

  inputs = {
    crane = {
      url = "github:ipetkov/crane";
      inputs = {
        nixpkgs.follows = "nixpkgs";
        flake-utils.follows = "flake-utils";
      };
    };
    flake-utils.url = "github:numtide/flake-utils";
    nixpkgs.url = "github:NixOS/nixpkgs";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs = {
        nixpkgs.follows = "nixpkgs";
        flake-utils.follows = "flake-utils";
      };
    };
  };

  outputs = {
    self,
    crane,
    flake-utils,
    nixpkgs,
    rust-overlay,
  }:
    flake-utils.lib.eachDefaultSystem (system: let
      pkgs = import nixpkgs {
        inherit system;
        overlays = [(import rust-overlay)];
      };

      code = pkgs.vscode-with-extensions.override {
        vscode = pkgs.vscodium;
        vscodeExtensions = with pkgs.vscode-extensions; [
          rust-lang.rust-analyzer
          vadimcn.vscode-lldb
          bungcip.better-toml
          bbenoist.nix
        ];
      };
      # Target musl when building on 64-bit linux
      buildTarget =
        {"x86_64-linux" = "x86_64-unknown-linux-musl";}.${system}
        or (pkgs.rust.toRustTargetSpec pkgs.stdenv.hostPlatform);
      rustToolchain = pkgs.rust-bin.stable.latest.default.override {
        targets = [
          buildTarget
          (pkgs.rust.toRustTargetSpec pkgs.stdenv.hostPlatform)
        ];
      };

      # Set-up build dependencies and configure rust
      craneLib = (crane.mkLib pkgs).overrideToolchain rustToolchain;

      # Shamelessly stolen from:
      # https://github.com/fedimint/fedimint/blob/66519d5e978e22bb10a045f406101891e1bb7eb5/flake.nix#L99
      filterSrcWithRegexes = regexes: src: let
        basePath = toString src + "/";
      in
        pkgs.lib.cleanSourceWith {
          filter = (
            path: type: let
              relPath = pkgs.lib.removePrefix basePath (toString path);
              includePath =
                (type == "directory")
                || pkgs.lib.any
                (re: builtins.match re relPath != null)
                regexes;
            in
              # uncomment to debug:
              # builtins.trace "${relPath}: ${lib.boolToString includePath}"
              includePath
          );
          inherit src;
        };

      cargo-details = pkgs.lib.importTOML ./Cargo.toml;
      binary-name = cargo-details.package.name;
      commonArgs = {
        nativeBuildInputs = with pkgs; [pkg-config];
        CARGO_BUILD_TARGET = buildTarget;
      };

      # Compile and cache only cargo dependencies
      dep-files-filter = ["Cargo.lock" "Cargo.toml" ".*/Cargo.toml"];
      cargo-deps = craneLib.buildDepsOnly (commonArgs
        // {
          src = filterSrcWithRegexes dep-files-filter ./.;
          pname = "${binary-name}-deps";
        });

      # Compile and cache only workspace code (seperately from 3rc party dependencies)
      package-file-filter = dep-files-filter ++ [".*\.rs"];
      cargo-package = craneLib.buildPackage (commonArgs
        // {
          inherit cargo-deps;
          src = filterSrcWithRegexes package-file-filter ./.;
          pname = binary-name;
        });
    in {
      checks = {
        # inherit # Comment in when you want tests to run on every new shell
        #   cargo-package
        #   ;
      };
      devShells.default = pkgs.mkShell {
        RUSTFLAGS = "--cfg uuid_unstable";
        nativeBuildInputs =
          (with pkgs; [
            # rust specific
            cargo-audit
            cargo-auditable
            cargo-cross
            cargo-deny
            cargo-outdated
            rust-analyzer

            # Editor stuffs
            code
            helix
            lldb
            rust-analyzer

            # Other tooling
            docker-compose
            postgresql_15
            sqlx-cli
            pgcli # Colorized syntax highlighted pgsql
          ])
          ++ [
            # Packages made in this flake
            rustToolchain
            # cargo-package # Comment in when you want tests to run on every new shell
          ]
          ++ pkgs.lib.optionals (pkgs.stdenv.isLinux) (with pkgs; [cargo-watch]); # Currently broken on macOS

        shellHook = ''
          ${rustToolchain}/bin/cargo --version
          ${pkgs.helix}/bin/hx --version
          ${pkgs.helix}/bin/hx --health rust
        '';
      };
      packages = {
        rust = cargo-package;
        docker = pkgs.dockerTools.buildImage {
          name = binary-name;
          tag = "v${cargo-details.package.version}";
          extraCommands = ''mkdir -p data'';
          config = {
            Cmd = "--help";
            Entrypoint = ["${cargo-package}/bin/${binary-name}"];
          };
        };
      };
      packages.default = cargo-package;

      # Now `nix fmt` works!
      formatter = pkgs.alejandra;
    });
}
