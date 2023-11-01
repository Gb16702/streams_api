pub struct EnvironmentVariables {
    server_address: String,
    server_port: String,
    database_url: String
}

pub struct AppState {
    pub db: sqlx::Pool<sqlx::Postgres>
}

impl EnvironmentVariables {
    pub fn new() -> Self {
            return Self {
                server_address: String::new(),
                server_port: String::new(),
                database_url: String::new()
            };
    }

    pub fn get_server_address(&self) -> &String {
        return &self.server_address;
    }

    pub fn set_server_address(&mut self, server_address: String) -> () {
        self.server_address = server_address;
    }

    pub fn get_server_port(&self) -> &String {
        return &self.server_port;
    }

    pub fn set_server_port(&mut self, server_port: String) -> () {
        self.server_port = server_port;
    }

    pub fn get_database_url(&self) -> &String {
        return &self.database_url;
    }

    pub fn set_database_url(&mut self, database_url: String) -> () {
        self.database_url = database_url;
    }
}
