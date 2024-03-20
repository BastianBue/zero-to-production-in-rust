use actix_web::{ web, App, HttpResponse, HttpServer, Responder, HttpRequest};

async fn index(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    HttpResponse::Ok().body(format!("Hello {}", name))
}

async fn health_check() -> impl Responder{
    return HttpResponse::Ok()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/health_check", web::get().to(health_check))
            .route("/{name}", web::get().to(index))
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}