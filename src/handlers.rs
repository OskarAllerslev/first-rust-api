use core::f64;

// import our crates
use axum::{Json, extract::{Path, Query}};
use reqwest::{ StatusCode};
use yahoo_finance_api as yahoo;
use chrono::{Utc, TimeZone};

use crate::types::{StockObservation, DataQueryParams, EmaQueryParams};
use crate::math::calculate_ema;

// this is the most basic handler

pub async fn alive_check() -> &'static str {
    "API is alive"
}




// we create the handler
pub async fn fetch_stock_data(
    ticker: &str, 
    interval: &str, 
    range: &str
) -> Result<Vec<StockObservation>, StatusCode> {
    let provider = yahoo::YahooConnector::new().unwrap();

    let response = provider.get_quote_range(ticker, interval, range)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?; 

    let quotes = response.quotes()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    let observations: Vec<StockObservation> = quotes 
        .iter()
        .map(|quote| {
            let date = Utc.timestamp_opt(quote.timestamp as i64, 0)
                .unwrap()
                .format("%Y-%m-%d")
                .to_string();
            
            StockObservation {
                date, 
                close: quote.close
            }
        })
        .collect();

    Ok(observations)
}

pub async fn get_ema(
    Path(ticker): Path<String>, 
    Query(params): Query<EmaQueryParams>,
) -> Result<Json<f64>, StatusCode> {
    // vi henter data, husker at kalde parametre med deres referencer og ikke kopiere dem over
    let data = fetch_stock_data(&ticker, &params.interval, &params.range)
        .await?; // vi skal huske at vente, og ? gør at vi fejler hvis vi ikke modtager noget
    // vi skal håndtere hvis data er tom
    if data.is_empty() { return Err(StatusCode::NOT_FOUND)}
    let final_ema = calculate_ema(&data, params.smoothing_constant);
    Ok(Json(final_ema))
}

















// der hvor vi laver de faktiske api endpoints ----

pub async fn get_ticker_data(
    Path(ticker): Path<String>,
    Query(params): Query<DataQueryParams>, 
) -> Result<Json<Vec<StockObservation>>, StatusCode> {
    let data = fetch_stock_data(&ticker, &params.interval, &params.range).await?;
    Ok(Json(data))
}
