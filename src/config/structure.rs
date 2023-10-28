pub struct EnvironmentVariables {
    server_url: String,
    server_port: String,
}

impl EnvironmentVariables {
    pub fn new() -> Self {
        return Self {
            server_url: String::new(),
            server_port: String::new(),
        };
    }

    pub fn get_server_url(&self) -> &String {
        return &self.server_url;
    }

    pub fn set_server_url(&mut self, server_url: String) {
        self.server_url = server_url;
    }

    pub fn get_server_port(&self) -> &String {
        return &self.server_port;
    }

    pub fn set_server_port(&mut self, server_port: String) {
        self.server_port = server_port;
    }
}