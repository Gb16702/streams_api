use actix_web::{App, HttpServer, web};


#[actix_web::main]
pub async fn handle_server_start(environment_variables: crate::EnvironmentVariables) -> Result<(), Box<dyn std::error::Error>> {
    use crate::routes::login_routes;
    use crate::database::connect::connect_to_database;

    let server: String = format!("{}:{}", environment_variables.get_server_address(), environment_variables.get_server_port());
    let pool_result = connect_to_database(environment_variables.get_database_url()).await;

     return match pool_result {
        Ok(pool) => {
            HttpServer::new(move || {
                App::new()
                    .app_data(web::Data::new(crate::config::structure::AppState {
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