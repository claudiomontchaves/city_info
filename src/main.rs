use app_properties::AppProperties;
use city_info::CityInfo;
use reqwest;
use reqwest::Error;
use reqwest::Response;
use reqwest::StatusCode;
use std::env;

mod city_info;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let city: &str = &args[1];

    let properties: AppProperties = AppProperties::new();
    let service_url = properties.get("service_url");
    let api_key = properties.get("api_key");

    let url = prepare_service_url(service_url, api_key, city);

    let response = get_city_info(url).await.unwrap();

    match response.status() {
        StatusCode::OK => {
            match response.json::<CityInfo>().await {
                Ok(city_info) => print_city_info(&city_info),
                Err(_) => println!("Response with wrong format!"),
            };
        }
        StatusCode::UNAUTHORIZED => {
            println!("Invalid token!");
        }
        other => {
            panic!("Unexpected status code: {:?}", other);
        }
    };
}

fn prepare_service_url(service_url: String, api_key: String, city: &str) -> String {
    format!(
        "{service_url_}?key={api_key_}&q={city_}&aqi=no",
        service_url_ = service_url,
        api_key_ = api_key,
        city_ = city
    )
}

async fn get_city_info(url: String) -> Result<Response, Error> {
    let client = reqwest::Client::new();
    client
        .get(url)
        .header("Content-Type", "application/json")
        .header("Accept", "application/json")
        .send()
        .await
}

fn print_city_info(city_info: &CityInfo) {
    println!("City:        {}", city_info.location.name);
    println!("Region:      {}", city_info.location.region);
    println!("Country:     {}", city_info.location.country);
    println!("Latitude:    {}", city_info.location.lat);
    println!("Longitude:   {}", city_info.location.lon);
    println!("Local Time:  {}", city_info.location.localtime);
    println!("Condition:   {}", city_info.current.condition.text);
    println!("Temperature: {} Celcius", city_info.current.temp_c);
    println!("Feels like:  {} Celcius", city_info.current.feelslike_c);
    println!("Wind:        {} km/h", city_info.current.wind_kph);
    println!("Wind Dir:    {} km/h", city_info.current.wind_dir);
}

#[cfg(test)]
mod test;
