mod server {
    pub mod server_start;
}

use std::env::{self, VarError};

#[derive(Debug)]
pub struct EnvironmentVariables {
    server_url: String,
    server_port: String,
}

fn load_env_variables() -> Result<EnvironmentVariables, VarError> {
    use dotenv::dotenv;
    dotenv().ok().expect("Echec du chargement du fichier .env");

    let server_url: Result<String, VarError> = env::var("SERVER_URL");
    let server_port: Result<String, VarError> = env::var("SERVER_PORT");
    return Ok(EnvironmentVariables {
        server_url: server_url?,
        server_port: server_port?,
    });
}

fn main() -> std::io::Result<()> {
    use crate::server::server_start::handle_server_start;

    let env_variables: EnvironmentVariables = load_env_variables().unwrap();

    return handle_server_start(env_variables);
}
