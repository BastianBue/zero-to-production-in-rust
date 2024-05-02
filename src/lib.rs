use actix_web::web::Form;
use actix_web::{get, post, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello, World!")
}

#[derive(Deserialize)]
struct FormData {
    name: String,
    email: String,
}

#[post("/signup")]
async fn signup(form: Form<FormData>) -> impl Responder {
    HttpResponse::Ok().body(format!("Hello, {}, {}!", form.name, form.email))
}

#[get("/health_check")]
async fn health_check() -> impl Responder {
    return HttpResponse::Ok();
}

pub async fn run() -> Result<(), std::io::Error> {
    HttpServer::new(|| App::new().service(index).service(health_check))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
