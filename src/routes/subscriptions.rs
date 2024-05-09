use actix_web::web::Form;
use actix_web::{post, HttpResponse, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
struct FormData {
    name: String,
    email: String,
}

#[post("/signup")]
pub async fn signup(form: Form<FormData>) -> impl Responder {
    HttpResponse::Ok().body(format!("Hello, {}, {}!", form.name, form.email))
}
