


use serde::{Serialize,Deserialize};
use sqlx::{FromRow};

#[derive(Serialize,Deserialize,FromRow)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub last_name:String,
}
