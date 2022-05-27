use std::collections::HashMap;
use reqwest::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Serialize, Deserialize, Debug)]
struct Weather {
    name : Vec<Name>,
    description: String,
    main: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Name {
    name : String,
    timezone: f32,
}

#[derive(Serialize, Deserialize, Debug)]
struct Main {
    temp: f32,
    temp_min: f32,
    temp_max: f32,
    feels_like: f32,
    weather: Weather 
}


#[derive(Serialize, Deserialize, Debug)]
struct Items<W> {
    items: Vec<W>,
}

#[derive(Serialize, Deserialize, Debug)]
struct APIResponse {
    mains: Items<Main>,
}

fn print_reports(mains: Vec<&Main>) {
    for main in mains {
        println!("🔥 {}", main.temp);
        println!("💿 {}", main.weather.description);
        println!(
            "🕺 {}",
            main
                .weather
                .name
                .iter()
                .map(|name| name.name.to_string())
                .collect::<String>()
        );
        println!("🌎 {}", main.temp);
        println!("---------")
    }
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let search_query = &args[1];    
    let url = format!("
    http://api.openweathermap.org/data/2.5/weather?q={query}&units=metric&appid=a2290f5132b80143df242aa1fe7a093d",
    query = search_query
);
    let client = reqwest::Client::new();
    let response = client
        .get(url)
        .header(CONTENT_TYPE, "application/json")
        .header(ACCEPT, "application/json")
        .send()
        .await
        // each response is wrapped in a `Result` type
        // we'll unwrap here for simplicity
        .unwrap();
    match response.status() {
        reqwest::StatusCode::OK => {
            match response.json::<APIResponse>().await {
                Ok(parsed) => print_reports(parsed.mains.items.iter().collect()),
                Err(_) => println!("Hm, the response didn't match the shape we expected."),
            };
        }
    };
    println!("It is working bruv! {:?}", response);

    
}