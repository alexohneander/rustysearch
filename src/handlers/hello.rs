use actix_web::{get, HttpResponse, Responder};

#[get("/")]
pub async fn say_hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}