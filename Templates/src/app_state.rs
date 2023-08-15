

use sqlx::postgres::{PgPool};


#[derive(Clone)]
pub struct AppState{
    pub pool: PgPool,
}

impl AppState {
    pub fn new(pool:PgPool)->Self{
        Self{
            pool:pool,
        }
    }

}
