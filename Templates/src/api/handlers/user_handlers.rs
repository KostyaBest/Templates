

use actix_web::{web,get,post,put,delete,HttpResponse,Responder,http::header::ContentType};

use sqlx::{FromRow};
use serde::{Serialize,Deserialize};


//OWN
use crate::app_state::{AppState};

#[derive(Serialize,FromRow)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub last_name:String,
}

#[derive(Deserialize)]
pub struct NameLastNameForm {
    pub name:String,
    pub last_name:String,
}



#[get("/users")]
pub async fn user_index(app_state: web::Data<AppState>) -> impl Responder {
    match sqlx::query_as::<_,User>("SELECT * from users")
        .fetch_one(&app_state.pool)
        .await
    {
        Ok(users)=>Some(HttpResponse::Ok().json(users)),
        Err(_) =>Some(HttpResponse::NotFound().json("No User found")),
    }
}

#[post("/users")]
pub async fn user_store(app_state: web::Data<AppState>,web::Form(form): web::Form<NameLastNameForm>)-> impl Responder {
    sqlx::query("INSERT INTO users(name,last_name) VALUES($1,$2);")
        .bind(form.name.as_str())
        .bind(form.last_name.as_str())
        .execute(&app_state.pool)
        .await;
   
    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body("Done")
}

#[get("/users/{id}")]
pub async fn user_show(id: web::Path<i32>,app_state:web::Data<AppState>)->impl Responder{
    let user = sqlx::query_as::<_,User>("SELECT * from  users where id=$1;")
        .bind(id.into_inner())
        .fetch_one(&app_state.pool)
        .await;
    match user {
        Ok(user)=>Some(HttpResponse::Ok().json(user)),
        Err(_) =>Some(HttpResponse::NotFound().json("No User found")),
    }
}

#[put("/users/{id}")]
pub async fn user_update(id: web::Path<i32>,web::Form(form):web::Form<NameLastNameForm>,app_state:web::Data<AppState>)->impl Responder{
    sqlx::query("UPDATE users SET name = $1, SET last_name=$2 WHERE id=$3;")
        .bind(form.name)
        .bind(form.last_name)
        .bind(id.into_inner())
        .fetch_one(&app_state.pool)
        .await;

    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body("Done")
}

#[delete("/users/{id}")]
pub async fn user_destroy(id: web::Path<i32>,app_state:web::Data<AppState>)->impl Responder{
        sqlx::query("DELETE FROM users WHERE id = $1;")
        .bind(id.into_inner())
        .fetch_one(&app_state.pool)
        .await;

    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body("Done")
}
