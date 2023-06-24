mod store;

use actix_web::{ App, HttpServer };
use store::create_store_resource;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(create_store_resource())
    })
    .bind(("127.0.0.1", 8084))?
    .run()
    .await
}
