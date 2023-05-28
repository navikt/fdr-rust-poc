CREATE TABLE IF NOT EXISTS person (
    id UUID NOT NULL PRIMARY KEY
);

CREATE TYPE identity_type AS ENUM (
    'PassportId',
    'SocialSecurityNumber'
);

CREATE TABLE IF NOT EXISTS identities (
    id UUID NOT NULL PRIMARY KEY,
    id_type identity_type NOT NULL,
    id_value TEXT NOT NULL,
    person UUID NOT NULL REFERENCES person(id),
    UNIQUE (id_value, id_type)
);
CREATE INDEX IF NOT EXISTS idx_identities_person ON identities(person);
