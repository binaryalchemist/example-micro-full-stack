mod data;

use actix_web::{ web, HttpResponse, Resource };
use data::{ from_file_to_promos };

async fn search_promotions() -> HttpResponse {
    let promotions_result = from_file_to_promos();

    match promotions_result {
        Ok(p) => HttpResponse::Ok().json(&p),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string())
    }
}

pub fn create_promotions_resource() -> Resource {
    return web::resource("/promotions")
        .route(web::get().to(search_promotions));
}
