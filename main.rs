use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;
use reqwest;
use std::collections::HashMap;

#[derive(Deserialize)]
struct BinanceResponse {
    price: String,
}


#[derive(Deserialize)]
struct CoinGeckoResponse {
    tether: HashMap<String, f64>,
}

async fn get_binance_rate() -> Option<f64> {
    let api_url = "https://api.binance.com/api/v3/ticker/price?symbol=USDTARS";
    if let Ok(response) = reqwest::get(api_url).await {
        if let Ok(data) = response.json::<BinanceResponse>().await {
            return data.price.parse::<f64>().ok();
        }
    }
    None
}

async fn get_coingecko_rate() -> Option<f64> {
    let api_url = "https://api.coingecko.com/api/v3/simple/price?ids=tether&vs_currencies=ars";
    if let Ok(response) = reqwest::get(api_url).await {
        if let Ok(data) = response.json::<CoinGeckoResponse>().await {
            return data.tether.get("ars").cloned();
        }
    }
    None
}

async fn get_best_usdtars() -> impl Responder {
    let binance_rate = get_binance_rate().await;
    let coingecko_rate = get_coingecko_rate().await;

    let rates: Vec<f64> = vec![binance_rate, coingecko_rate]
        .into_iter()
        .filter_map(|rate| rate)
        .collect();

    if let Some(best_rate) = rates.iter().cloned().max_by(|a, b| a.partial_cmp(b).unwrap()) {
        HttpResponse::Ok().json(serde_json::json!({ "Best USDT/ARS Rate": best_rate }))
    } else {
        HttpResponse::InternalServerError().body("Failed to fetch data from APIs")
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/get_best_usdtars", web::get().to(get_best_usdtars))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
