use actix_web::{App, get, HttpResponse, HttpServer, Responder};
use crate::EnvironmentVariables;

#[get("/")]
async fn index() -> impl Responder {
    return HttpResponse::Ok().body("Success");
}

#[actix_web::main]
pub async fn handle_server_start(environment_variables: EnvironmentVariables) -> std::io::Result<()> {
    let server: String = format!("{}:{}", environment_variables.server_url, environment_variables.server_port);

    return HttpServer::new(|| {
        App::new()
            .service(index)
    }).bind(server)?.run().await;
}
