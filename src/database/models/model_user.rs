#[derive(Debug)]
pub struct User {
    email: String,
    password: String,
    is_admin: bool
}

impl User {
    pub fn new(email: String, password: String, is_admin: bool) -> Self {
        User {
            email,
            password,
            is_admin
        }
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
