use dotenv::dotenv;
use reqwest;
use std::{collections::HashMap, env};
use serde::Deserialize;
#[derive(Debug, Deserialize, )]
pub struct  ApiResponse{
conversion_rates: HashMap<String, f64>
}
pub async fn get_usdt_ngn_rate() -> Result<f64, Box<dyn std::error::Error>>{
dotenv().ok();
let rate_url = env::var("EXCHANGE_RATE_API_KEY")?;
let res = reqwest::get(rate_url)
    .await?.json::<ApiResponse>()
    .await?;
let result = res.conversion_rates.get("NGN").unwrap();
Ok(*result)
}