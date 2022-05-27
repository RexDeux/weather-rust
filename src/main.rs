use serde_json::json;
use reqwest;
use serde::{Deserialize, Serialize};
use std::env;
use reqwest::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};

#[derive(Serialize, Deserialize, Debug)]
struct Weather {
    name : String,
    main: u8,
    timezone: u8,
    description: String,
}

fn print_report(weathers: Vec<&Weather>) {
    for weather in weathers {
        println!("🔥 {}", weather.name);
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
        .send()
        .await
        // each response is wrapped in a `Result` type
        // we'll unwrap here for simplicity
        .unwrap()
        .text()
        .await;
    println!("It is working bruv! {:?}", response);

    match response.status() {
        reqwest::StatusCode::OK => {
            match response.json::<APIResponse>().await {
                Ok(parsed) => print_report(parsed.tracks.items.iter().collect()),
                Err(_) => println!("Hm, the response didn't match the shape we expected."),
            };
        }
    };
}