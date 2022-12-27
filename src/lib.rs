use db_service::DbService;
use main_service::MainService;
use shuttle_secrets::SecretStore;

pub mod db_service;
pub mod main_service;

#[shuttle_service::main]
async fn init(
    #[shuttle_shared_db::Postgres] pool: sqlx::PgPool,
    #[shuttle_secrets::Secrets] secret_store: SecretStore,
) -> Result<MainService, shuttle_service::Error> {
    Ok(MainService {
        db_service: DbService { pool },
        secret_store,
    })
}
