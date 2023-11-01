mod config;
mod routes {
    pub mod login_routes;
}

mod server {
    pub mod server_start;
}

use crate::config::structure::EnvironmentVariables;
use std::env::{self, VarError};

fn load_env_variables() -> Result<EnvironmentVariables, VarError> {
    use dotenv::dotenv;
    dotenv().ok().ok_or_else(|| VarError::NotPresent)?;

    let mut env_variables: EnvironmentVariables = EnvironmentVariables::new();
    env_variables.set_server_address(env::var("SERVER_ADDRESS")?);
    env_variables.set_server_port(env::var("SERVER_PORT")?);
    env_variables.set_database_url(env::var("DATABASE_URL")?);

    return Ok(env_variables);
}

fn main() -> Result<(), std::io::Error> {
    use crate::server::server_start;

    return match server_start::handle_server_start(load_env_variables().unwrap()) {
        Ok(()) => {
            Ok(())
        },
        Err(err) => {
            eprintln!("Error: {}", err);
            Err(std::io::Error::new(std::io::ErrorKind::Other, "An error occurred"))
        }
    };
}
