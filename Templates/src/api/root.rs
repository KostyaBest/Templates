

use actix_web::{web, HttpResponse};


//OWN
#[path="../app_state.rs"]
mod app_state;

#[path="../models/mod.rs"]
mod models;


use app_state::{AppState};
use models::user::{User};

pub async fn root() -> String {
    "Server is up and running".to_string()
}



pub async fn get_user(path: web::Path<usize>, app_state: web::Data<AppState>) -> String {
    let user_id: usize = path.into_inner();

    "get user".to_string()
    //sqlx::query();
    //sqlx::query_as();
}
