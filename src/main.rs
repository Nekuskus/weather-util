use colored::Colorize;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
struct IpData {
    status: String,
    message: Option<String>,
    country: String,
    city: String,
    lat: f32,
    lon: f32,
    isp: String
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let location_data = reqwest::get("http://ip-api.com/json/?fields=status,message,country,city,lat,lon,isp")
        .await?
        .json::<IpData>()
        .await?;
    println!("{:?}", location_data);

    Ok(())
}