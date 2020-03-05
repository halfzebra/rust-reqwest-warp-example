use std::collections::HashMap;
use warp::{Filter};
use log::{info};
use std::time::{SystemTime};
use pretty_env_logger;

async fn fetch_data() -> Result<HashMap<String, String>, reqwest::Error> {
    let data_source_url = "https://httpbin.org/ip";
    info!("Sending a request to {}", data_source_url);
    let now = SystemTime::now();
    let res = reqwest::get(data_source_url)
        .await?
        .json::<HashMap<String, String>>()
        .await?;
    let elapsed = now.elapsed().unwrap();
    info!("Time elapsed doing a request {:?}ms", elapsed.as_millis());
    Ok(res)
}

async fn render_response() -> Result<impl warp::Reply, warp::Rejection> {
    let resp = fetch_data().await;
    match resp {
        Ok(data) => Ok(warp::reply::json(&data)),
        Err(_err) => Err(warp::reject()),
    }
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    // GET /* => 200 OK with JSON body
    let routes = warp::any().and_then(render_response);

    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;
}