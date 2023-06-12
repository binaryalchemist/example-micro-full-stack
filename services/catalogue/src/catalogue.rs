use actix_web::{ web, HttpResponse, Resource };
use serde::{Deserialize, Serialize};
use serde_json::from_str;
use std::fs;

#[derive(Deserialize, Serialize)]
pub struct CatalogueItem {
    name: String,
    distillery: String,
    price: f32
}

#[derive(Deserialize, Serialize)]
pub struct Catalogue {
    items: Vec<CatalogueItem>
}

fn read_catalogue_file() ->  Result<Catalogue, Box<dyn std::error::Error>> {
    let file_path = "data/catalogue.json";
    
    let file_data_string = fs::read_to_string(file_path)?;
    let catalogue: Catalogue = from_str(&file_data_string)?;

    Ok(catalogue)
}


async fn get_catalogue() -> HttpResponse {
    let promotions_result = read_catalogue_file();

    match promotions_result {
        Ok(p) => HttpResponse::Ok().json(&p),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string())
    }
}

pub fn create_catalogue_resource() -> Resource {
    return web::resource("/catalogue")
        .route(web::get().to(get_catalogue));
}
