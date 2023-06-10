use actix_web::{ web, HttpResponse, Resource };
use serde::{Deserialize, Serialize};
use serde_json::from_str;
use std::fs;

#[derive(Serialize, Deserialize)]
pub struct Coupon {
    value: f32,
    min_price: f32
}

#[derive(Serialize, Deserialize)]
pub struct Scope {
    name: String,
    values: Option<Vec<String>>
}

#[derive(Serialize, Deserialize)]
pub struct Promotion {
    id: i32,
    description: String,
    scope: Scope,
    coupon: Coupon
}

#[derive(Serialize, Deserialize)]
pub struct Promotions {
    promotions: Vec<Promotion>
}

fn from_file_to_promos() -> Result<Promotions, Box<dyn std::error::Error>> {
    let file_path = "data/promotions.json";
    let file_data_string = fs::read_to_string(file_path)?;
    let promos: Promotions = from_str(&file_data_string)?;

    Ok(promos)
}

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
