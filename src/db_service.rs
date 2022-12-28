pub struct DbService {
    pool: sqlx::PgPool,
}

impl DbService {
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self { pool }
    }
}
