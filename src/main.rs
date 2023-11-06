mod config;
mod database;
mod utils;
mod routes;
mod server;

use crate::config::structure;
use std::env::{self, VarError};

fn define_user() -> Result<Vec<structure::Cred>, VarError> {
    let mut arr: Vec<structure::Cred> = Vec::new();
    for x in 0..2 {
        match (env::var(format!("AE_{}",x)), env::var(format!("AP_{}",x))) {
            (Ok(e), Ok(p)) => {
                arr.push(structure::Cred {
                    user_email: e,
                    user_password: p,
                });
            }, _ => println!("Error: {:?}", VarError::NotPresent)
        }
    }
    return Ok(arr);
}
fn load_env_variables() -> Result<structure::EnvironmentVariables, VarError> {
    use dotenv::dotenv;
    dotenv().ok().ok_or_else(|| VarError::NotPresent)?;

    let mut env_variables: structure::EnvironmentVariables = structure::EnvironmentVariables::new();
    env_variables.set_server_address(env::var("SERVER_ADDRESS")?);
    env_variables.set_server_port(env::var("SERVER_PORT")?);
    env_variables.set_database_url(env::var("DATABASE_URL")?);

    return Ok(env_variables);
}

fn main() -> Result<(), std::io::Error> {
    use crate::server::server_start;

    let environment_variables: structure::EnvironmentVariables = load_env_variables().unwrap();

    return match server_start::handle_server_start(environment_variables, define_user) {
        Ok(()) => Ok(()),
        Err(err) => {
            eprintln!("Error: {}", err);
            Err(std::io::Error::new(std::io::ErrorKind::Other, "An error occurred"))
        }
    };
}
