

use actix_web::{web, HttpServer, App, middleware::Logger};
use sqlx::postgres::{PgPool,PgPoolOptions};

//use log;
//use dotenv::dotenv;

//OWN
mod api;

mod app_state;
use app_state::{AppState};
// use api::handlers::user_handlers::{
//     user_index,
//     user_store,
//     user_update,
//     user_show,
//     user_destroy,
// };
use crate::api::handlers::user_scope::{user_scope};
use api::handlers::user_scope;
use api::handlers::authontication_token::{AuthonticationToken};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    
    let db_url= dotenv::var("DATABASE_URL").unwrap();
    let app_host=dotenv::var("APP_HOST").unwrap(); 
    let app_port=dotenv::var("APP_PORT").unwrap().parse::<u16>().unwrap();

    let pool: PgPool = PgPoolOptions::new()
        .max_connections(5)
        .connect(db_url.as_str())
        //.connect_lazy(DB_URL.as_str())
        .await
        .unwrap();

    let app_state = AppState::new(pool.clone());

    HttpServer::new(move || {
        let logger = Logger::default();
        App::new()
            .wrap(logger)
            .app_data(web::Data::new(app_state.clone()))
            .app_data(web::Data::new(String::from("secret")))
            // .service(user_index)
            // .service(user_store)
            // .service(user_update)
            // .service(user_show)
            // .service(user_destroy)
            .service(user_scope())
    }).bind((app_host.as_str(),app_port))?
    .run()
    .await

}
