use serde::{Deserialize, Serialize};
use serde_json::{from_str};
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

fn read_promos_file() ->  Result<String, Box<dyn std::error::Error>> {
    let file_path = "sample/data.json";
    
    let file_data_string = fs::read_to_string(file_path);

    match file_data_string {
        Ok(f) => Ok(f),
        Err(e) => {
            println!("error reading promos file {}", e.to_string());
            
            Err(Box::new(e))
        }
    }
}

fn from_file_to_promos() -> Result<Promotions, Box<dyn std::error::Error>> {
    let file_data_string = read_promos_file()?;

    let promos = from_str(&file_data_string);

    match promos {
        Ok(p) => Ok(p),
        Err(e) => {
            println!("{}", file_data_string);
            println!("error converting promo string to object {}", e);

            Err(Box::new(e))
        }
    }
}

pub fn get_all() -> Result<Vec<Promotion>, Box<dyn std::error::Error>> {
    let promos = from_file_to_promos()?;

    Ok(promos.promotions)
}
