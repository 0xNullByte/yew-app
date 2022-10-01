#[allow(dead_code)]
pub struct Users{
    id: i32,
    username: String,
    password: String,
}

#[allow(dead_code)]
impl Users {
    pub fn new(id: i32, username: String, password: String) -> Self {
        Self{id: id, username: username, password: password}
    }
    pub fn add_user(){}
    pub fn del_user(){}
}