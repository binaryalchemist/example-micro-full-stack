mod api;

use warp::Filter;

#[tokio::main]
async fn main() {
    let promotions = warp::path("promotions")
        .map(|| api::controller::search_promotions());

    let routes = warp::get().and(promotions);

    warp::serve(routes)
        .run(([127, 0, 0, 1], 8082))
        .await;
}
