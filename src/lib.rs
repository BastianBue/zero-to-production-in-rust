use actix_web::{ App, HttpResponse, HttpServer, Responder, get};

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello, World!")
}

#[get("/health_check")]
async fn health_check() -> impl Responder {
    return HttpResponse::Ok();
}

pub async fn run() -> Result<(), std::io::Error> {
    HttpServer::new(|| { App::new().service(index).service(health_check) })
        .bind(("127.0.0.1", 8080))?
        .run().await
}