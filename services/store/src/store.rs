use actix_web::{ web, HttpResponse, Resource };
use serde::{ Serialize, Deserialize };
use serde::ser::StdError;


#[derive(Serialize, Deserialize)]
struct Coupon {
    value: f32,
    min_price: f32
}

#[derive(Serialize, Deserialize)]
struct Scope {
    name: String,
    values: Option<Vec<String>>
}

#[derive(Serialize, Deserialize)]
struct Promotion {
    id: i32,
    description: String,
    scope: Scope,
    coupon: Coupon
}

#[derive(Serialize, Deserialize)]
struct Promotions {
    promotions: Vec<Promotion>
}


#[derive(Serialize, Deserialize)]
struct CatalogueItem {
    name: String,
    distillery: String,
    price: f32
}

#[derive(Serialize, Deserialize)]
struct Catalogue {
    items: Vec<CatalogueItem>
}

#[derive(Serialize, Deserialize)]
pub struct Store {
    name: String,
    description: String,
    catalogue: Catalogue
}

async fn fetch_catalogue() -> Result<Catalogue, Box<dyn StdError>> {
    let get_catalogue_url = String::from("http:://127.0.0.1::8086/catalogue");

    let response = reqwest::get(&get_catalogue_url)
        .await?
        .json::<Catalogue>()
        .await?;
    
    Ok(response)
}

async fn get_store_data() -> Result<Store, Box<dyn StdError>> {
    let scotch_catalogue = fetch_catalogue().await?;

    let store = Store {
        name: String::from("Scotch Store"),
        description: String::from("Store With A Few Scotches"),
        catalogue: scotch_catalogue
    };

    Ok(store)
}

async fn search_store() -> HttpResponse {
    let store_result = get_store_data().await;

    match store_result {
        Ok(p) => HttpResponse::Ok().json(&p),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string())
    }
}

pub fn create_store_resource() -> Resource {
    return web::resource("/store")
        .route(web::get().to(search_store));
}
