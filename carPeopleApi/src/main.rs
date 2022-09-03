use rand::Rng;
use std::collections::HashMap;
use warp::Filter;

async fn calculate_dissel_usage_for_distance(
    param: HashMap<String, String>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let mut distance = 0.0;
    let mut year_of_production = 0;
    let mut fuel_usage = 0.0;
    let mut fuel_consumotion = 0.0;
    for (key, value) in &param {
        if key == "distance" {
            let my_string = value.to_string();
            let dist = my_string.parse::<f64>().unwrap_or(0.0);
            distance = dist;
        }
        if key == "yearOfProduction" {
            let my_stringx = value.to_string();
            let yer = my_stringx.parse::<i32>().unwrap_or(0);
            year_of_production = yer;
        }
        if key == "fuelUsagePer100KM" {
            let my_stringxx = value.to_string();
            let us = my_stringxx.parse::<f64>().unwrap_or(0.0);
            fuel_usage = us;
        }
    }
    if distance > 0.0 && fuel_usage > 0.0 {
        fuel_consumotion = (distance / 100.0) * fuel_usage;
    }
    if fuel_consumotion <= 0.0 {
        Ok(format!("{{\"Error\": \"invalid input\"}}"))
    } else {
        Ok(format!("{{\"fuelUsage\": {}}}", fuel_consumotion))
    }
}

async fn probability_of_unit_injector_fail(
    param: HashMap<String, String>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let mut posibility = rand::thread_rng().gen_range(0, 100) as f64;
    posibility = posibility / 100.0;
    Ok(format!("{{\"failProbability\": {:#?}}}", posibility))
}
#[tokio::main]
async fn main() {
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
