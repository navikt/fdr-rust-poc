use rocket::{
	fairing::{self, AdHoc},
	Build, Rocket,
};
use rocket_db_pools::{sqlx, Database};
use tracing::error;

#[derive(Database)]
#[database("DB")]
pub(crate) struct Db(sqlx::PgPool);

async fn run_migrations(rocket: Rocket<Build>) -> fairing::Result {
	match Db::fetch(&rocket) {
		Some(db) => match sqlx::migrate!("./database-migrations").run(&**db).await {
			Ok(_) => Ok(rocket),
			Err(e) => {
				error!("Failed to initialize SQLx database: {}", e);
				Err(rocket)
			},
		},
		None => Err(rocket),
	}
}

pub fn stage() -> AdHoc {
	AdHoc::on_ignite("SQLx Stage", |rocket| async {
		rocket
			.attach(Db::init())
			.attach(AdHoc::try_on_ignite("SQLx Migrations", run_migrations))
	})
}
