use actix_web::{web, App, HttpServer};

mod api;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/", web::get().to(api::health_check)))
        .bind("127.0.0.1:8000")?
        .run()
        .await
}
