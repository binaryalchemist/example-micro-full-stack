mod promotions;

use actix_web::{ App, HttpServer };
use promotions::create_promotions_resource;

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(create_promotions_resource())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
