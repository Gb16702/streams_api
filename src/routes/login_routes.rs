use actix_web::{web, post, Responder, HttpResponse, get };
use serde_json;
#[get("/")]
pub async fn index() -> impl Responder {
        return  HttpResponse::Ok().body("Success");
}

#[derive(serde::Serialize)]
struct ErrorResponse {
        error: &'static str,
}
#[post("/login")]
pub async fn login(body: web::Json<crate::config::structure::Cred>) -> impl Responder {
        let email = &body.user_email;
        let password = &body.user_password;

        use crate::utils::validate_email::validate_email;

        match validate_email(email) {
                Ok(_) => (),
                Err(_err) => {
                        let error_response = ErrorResponse {
                                error: "L'adresse mail est invalide",
                        };
                        return HttpResponse::BadRequest().content_type("application/json").body(
                                serde_json::to_string(&error_response).unwrap()
                        )
                }
        }



        return HttpResponse::Ok().content_type("application/json").body(
                serde_json::to_string(&crate::config::structure::Cred {
                        user_email: String::from(email),
                        user_password: String::from(password)
                }).unwrap()
        );
}
