



#[derive(Serialize,FromRow)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub last_name:String,
}

