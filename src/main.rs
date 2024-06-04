use colored::Colorize;
use dotenv::dotenv;
use serde::Deserialize;
use std::collections::HashMap;
use serde_json::Value;


// This one's small enough to be worth a declaration
#[derive(Deserialize, Debug)]
struct IpData {
    status: String,
    message: Option<String>,
    country: String,
    city: String,
    lat: f32,
    lon: f32,
    isp: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let location_data =
        reqwest::get("http://ip-api.com/json/?fields=status,message,country,city,lat,lon,isp")
            .await?
            .json::<IpData>()
            .await?;
    println!("{:?}", location_data);

    let openweather_api_token =
        std::env::var("OPENWEATHER_KEY").expect("OPENWEATHER_KEY must be set.");

    let weather_data: Value = serde_json::from_str(
        &reqwest::get(format!(
            "https://api.openweathermap.org/data/2.5/weather?lat={}&lon={}&appid={}",
            location_data.lat, location_data.lon, openweather_api_token
        ))
        .await?
        .text()
        .await?,
    )?;
    println!("{:#?}", weather_data);

    Ok(())
}
