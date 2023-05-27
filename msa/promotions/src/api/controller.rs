use crate::api::data::get_all;
use warp::reply::{ with_status, WithStatus };
use warp::hyper::StatusCode;

pub fn search_promotions() -> WithStatus<String> {
    let promotions_result = get_all();

    match promotions_result {
        Ok(p) => with_status(serde_json::to_string(&p).unwrap(), StatusCode::OK),
        Err(e) => with_status(serde_json::to_string(&e.to_string()).unwrap(), StatusCode::INTERNAL_SERVER_ERROR)
    }
}
