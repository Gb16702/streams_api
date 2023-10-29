use crate::EnvironmentVariables;
use actix_web::{App, HttpServer, web};

#[actix_web::main]
pub async fn handle_server_start(environment_variables: EnvironmentVariables) -> std::io::Result<()> {
    use crate::routes::login_routes;

    let server: String = format!("{}:{}", environment_variables.get_server_address(), environment_variables.get_server_port());

    return HttpServer::new(|| {
        App::new()
        .service(web::scope("/api").service(login_routes::login)
            .service(login_routes::index)
        )
    }).bind(server)?.run().await;
}
