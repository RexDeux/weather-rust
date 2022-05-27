use serde_json::json;
use reqwest;

struct Report {
    coord: u8,
    main: u8,
    timezone: u8,
    description: String,
}

#[tokio::main]
async fn main() {
    let url = format!("
    http://api.openweathermap.org/data/2.5/weather?q={query}&units=metric&appid=a2290f5132b80143df242aa1fe7a093d",
    query = "Braga"
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
}
