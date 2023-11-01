use actix_web::{App, HttpServer, web};
use crate::EnvironmentVariables;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::error::Error;

pub struct AppState {
    pub db: Pool<Postgres>
}

async fn migrate_database(pool: &Pool<Postgres>) -> Result<(), sqlx::Error> {
    sqlx::migrate!("./migrations").run(pool).await?;
    Ok(())
}

async fn connect_to_database(database_url: &str) ->  Result<Pool<Postgres>, Box<dyn Error>> {
    let pool: Pool<Postgres> = PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url).await.expect("Failed to connect to database");

    println!("La base de données est connectée");

    migrate_database(&pool).await?;

    println!("Les migrations ont été effectuées");

    return Ok(pool);
}

#[actix_web::main]
pub async fn handle_server_start(environment_variables: EnvironmentVariables) -> Result<(), Box<dyn Error>> {
    use crate::routes::login_routes;

    let server: String = format!("{}:{}", environment_variables.get_server_address(), environment_variables.get_server_port());
    let pool_result = connect_to_database(environment_variables.get_database_url()).await;

     return match pool_result {
        Ok(pool) => {
            HttpServer::new(move || {
                App::new()
                    .app_data(web::Data::new(AppState {
                        db: pool.clone()
                    }))
                    .service(web::scope("/api")
                        .service(login_routes::login)
                        .service(login_routes::index)
                    )
            }).bind(server)?.run().await?;
            Ok(())
        }
        Err(err) => {
            Err(err)
        }
    };
}