use std::collections::HashMap;

use warp::Filter;

async fn calculate_dissel_usage_for_distance(
    param: HashMap<String, String>,
) -> Result<impl warp::Reply, warp::Rejection> {
    Ok(format!("calculateDisselUsageForDistance {:#?}", param))
}

async fn probability_of_unit_injector_fail(
    param: HashMap<String, String>,
) -> Result<impl warp::Reply, warp::Rejection> {
    Ok(format!("probabilityOfUnitInjectorFail {:#?}", param))
}
#[tokio::main]
async fn main() {
    // GET /hello/warp => 200 OK with body "Hello, warp!"
    //let hello = warp::path!("hello" / String).map(|name| format!("Hello, {}!", name));

    let calculate_dissel_usage_for_distance = warp::get()
        .and(warp::path("calculateDisselUsageForDistance"))
        .and(warp::query::<HashMap<String, String>>())
        .and(warp::path::end())
        .and_then(calculate_dissel_usage_for_distance);

    let probability_of_unit_injector_fail = warp::get()
        .and(warp::path("probabilityOfUnitInjectorFail"))
        .and(warp::query::<HashMap<String, String>>())
        .and(warp::path::end())
        .and_then(probability_of_unit_injector_fail);

    let routes = calculate_dissel_usage_for_distance.or(probability_of_unit_injector_fail);

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
