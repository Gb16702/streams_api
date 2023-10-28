use actix_web::{post, Responder, HttpResponse, get};

#[get("/")]
pub async fn index() -> impl Responder {
        return HttpResponse::Ok().body("Success")
}

#[post("/login")]
pub async fn login() -> impl Responder {
        return HttpResponse::Ok().body("Success");
}
