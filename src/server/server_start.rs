use actix_web::{App, HttpServer, web};
use actix_cors::Cors;

#[actix_web::main]
pub async fn handle_server_start<F>(environment_variables: crate::structure::EnvironmentVariables, define_user: F) -> Result<(), Box<dyn std::error::Error>>
where
    F: Fn() -> Result<Vec<crate::structure::Cred>, std::env::VarError>
{
    use crate::routes::login_routes;
    use crate::database::connect::connect_to_database;
    use crate::utils::create_user::create_user;

    let server: String = format!("{}:{}", environment_variables.get_server_address(), environment_variables.get_server_port());
    let pool_result = connect_to_database(environment_variables.get_database_url()).await;

    for x in define_user().unwrap() {
        create_user(x.user_email, x.user_password,
            &pool_result.as_ref().unwrap()).await;
    }

     return match pool_result {
        Ok(pool) => {
            HttpServer::new(move || {
                let cors_middleware= Cors::default()
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header()
                    .max_age(3600);
                App::new()
                    .app_data(web::Data::new(crate::config::structure::AppState {
                        db: pool.clone()
                    }))
                    .wrap(cors_middleware)
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
