use serde::{Deserialize, Serialize};
// we create our handler for the stock data 
// we need to define what a row in our data looks like 
// we need to tell rust that we whould translate this struct from and to JSON
#[derive(Serialize, Deserialize)]
pub struct StockObservation {
    pub date: String,
    pub close: f64,
}

#[derive(serde::Deserialize)]
pub struct DataQueryParams{
    pub interval: String, // til fetch_stock_data
    pub range: String, // til fetch_stock_data
}

#[derive(serde::Deserialize)]
pub struct EmaQueryParams{
    pub interval: String, // til fetch_stock_data
    pub range: String, // til fetch_stock_data
    pub smoothing_constant: f64,
}