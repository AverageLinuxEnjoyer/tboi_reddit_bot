use db_service::DbService;
use main_service::MainService;

pub mod db_service;
pub mod main_service;

#[shuttle_service::main]
async fn init(
    #[shuttle_shared_db::Postgres] pool: sqlx::PgPool,
) -> Result<MainService, shuttle_service::Error> {
    Ok(MainService {
        db_service: DbService { pool },
    })
}
