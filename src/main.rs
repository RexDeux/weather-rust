use serde::{Deserialize,Serialize};
use reqwest;
use std::env;
use exitfailure::ExitFailure;
use reqwest::Url;
use structopt::StructOpt;


#[derive(StructOpt)]
struct Cli {
    city: String,
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub coord: Coord,
    pub weather: Vec<Weather>,
    pub base: String,
    pub main: Main,
    pub visibility: i64,
    pub wind: Wind,
    pub clouds: Clouds,
    pub dt: i64,
    pub sys: Sys,
    pub timezone: i64,
    pub id: i64,
    pub name: String,
    pub cod: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Coord {
    pub lon: f64,
    pub lat: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Weather {
    pub id: i64,
    pub main: String,
    pub description: String,
    pub icon: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Main {
    pub temp: f64,
    #[serde(rename = "feels_like")]
    pub feels_like: f64,
    #[serde(rename = "temp_min")]
    pub temp_min: f64,
    #[serde(rename = "temp_max")]
    pub temp_max: f64,
    pub pressure: i64,
    pub humidity: i64,
    #[serde(rename = "sea_level")]
    pub sea_level: Option<i32>,
    #[serde(rename = "grnd_level")]
    pub grnd_level: Option<i32>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Wind {
    pub speed: f64,
    pub deg: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Rain {
    #[serde(rename = "1h")]
    pub n1h: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Clouds {
    pub all: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sys {
    #[serde(rename = "type")]
    pub type_field: i64,
    pub id: i64,
    pub country: String,
    pub sunrise: i64,
    pub sunset: i64,
}


#[tokio::main]
async fn main() -> Result<(), ExitFailure> {
    let args = Cli::from_args();
    let response = Root::get(&args.city).await?;
    let temp_cel = response.main.temp;
    let hum = response.main.humidity;
    let max_temp = response.main.temp_max;
    let min_temp = response.main.temp_min;
    println!(
        "Right now in {} the temperature is {:.2} C with a level of humidity of {}. The maximum temperature will be {} and the minimum {}",
        args.city, temp_cel, hum, max_temp, min_temp
    );
    Ok(())
}

impl Root {
    async fn get(city: &String) -> Result<Self, ExitFailure> {
        let url: String = format!(
            "http://api.openweathermap.org/data/2.5/weather?q={}&units=metric&appid=a2290f5132b80143df242aa1fe7a093d",
            city,
        );
        let url: Url = Url::parse(&*url)?;

        let resp = reqwest::get(url).await?.json::<Root>().await?;
        Ok(resp)
    }
}
