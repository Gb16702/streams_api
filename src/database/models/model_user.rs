#[derive(Debug)]
pub struct User {
    id: i32,
    email: String,
    password: String,
    is_admin: bool
}

impl User {
    pub fn new(id: i32, email: String, password: String, is_admin: bool) -> Self {
        User {
            id,
            email,
            password,
            is_admin
        }
    }

    pub fn get_id(&self) -> &i32 {
        return &self.id;
    }

    pub fn get_email(&self) -> &String {
        return &self.email;
    }

    pub fn get_password(&self) -> &String {
        return &self.password;
    }

    pub fn get_is_admin(&self) -> &bool {
        return &self.is_admin;
    }
}
