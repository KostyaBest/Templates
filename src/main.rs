
use std::io::Result;
use actix_web::{HttpServer,App,web};

#[path="./handlers/mod.rs"]
mod handlers;

use crate::handlers::example::{hello,echo,manual_hello};


#[actix_web::main]
async fn main() -> Result<()>{
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/hey",web::get().to(manual_hello))
    })
    .bind(("127.0.0.1",8080))?
    .run()
    .await
}
