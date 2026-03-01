use core::f64;

// import our crates
use axum::{Json, extract::{self, Path, Query}};
use reqwest::{ StatusCode};
use serde::{Deserialize, Serialize};
use yahoo_finance_api as yahoo;
use chrono::{Utc, TimeZone};

// this is the most basic handler

pub async fn alive_check() -> &'static str {
    "API is alive"
}


// we create our handler for the stock data 
// we need to define what a row in our data looks like 
// we need to tell rust that we whould translate this struct from and to JSON
#[derive(Serialize, Deserialize)]
pub struct StockObservation {
    pub date: String,
    pub close: f64,
}

#[derive(serde::Deserialize)]
pub struct QueryParams {
    pub interval: String,
    pub range: String,
}



// we create the handler
pub async fn get_ticker_data(
    extract::Path(ticker): Path<String>,
    extract::Query(params): Query<QueryParams>, 
) -> Result<Json<Vec<StockObservation>>, StatusCode> {
    let provider = yahoo::YahooConnector::new().unwrap();

    let response = provider.get_quote_range(&ticker, &params.interval, &params.range)
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

    Ok(Json(observations))
}

