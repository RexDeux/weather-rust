use std::hash::Hash;
use std::io;
use std::format;
use std::io::Read;
use reqwest::Error;
use std::collections::HashMap;

struct Report {
    name: String,
    temp: String,
    timezone: String,
    description: String,
}

fn main() {
    print_header();

    println!("Want to check the weather in (Braga)?");

    let mut city = String::new();
    io::stdin().read_line(&mut city).expect("Cannot read the input.");

    let weather = match process_json(&city) {
        Ok(report) => report,
        _ => String::from("An error occurred while processing.")
    };

    println!("{}", weather);
}

fn process_json(input: &str) -> Result<String, Error> {
    let html = api(&input)?;
    let formatted = match api(&input) {
        Some(report) => format!("The temp in {} is {} and the the weather is {}.", report.name, report.timezone, report.description),
        None => String::from("An error while parsing the weather report.")
    };
    Ok(formatted)
}

fn print_header() {
    println!("-------------------------");
    println!("         Gucci Gang");
    println!("-------------------------");
}

async fn api(city: &str) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!("http://api.openweathermap.org/data/2.5/weather?q={}&units=metric&appid=a2290f5132b80143df242aa1fe7a093d", city);
    let resp = reqwest::get(&url)
        .await?
        .json::<HashMap<String, String>>()
        .await?;
    Ok(())
}
