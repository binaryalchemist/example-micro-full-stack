mod catalogue;

use actix_web::{ App, HttpServer };
use catalogue::create_catalogue_resource;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(create_catalogue_resource())
    })
    .bind(("127.0.0.1", 8082))?
    .run()
    .await
}
