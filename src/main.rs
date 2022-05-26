use std::io;
use std::format;
use std::io::Read;
use reqwest::Error;
use scraper::html;
use scraper::{Html, Selector};

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
    let formatted = match api(&html) {
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

fn api(city: &str) -> Result<String, Error> {
    let url = format!("http://api.openweathermap.org/data/2.5/weather?q={}&units=metric&appid=a2290f5132b80143df242aa1fe7a093d", city);
    let mut res = reqwest::blocking::get(&url)?;
    let mut body = String::new();
    println!("Status: {}", res.status());
    println!("Headers:\n{:#?}", res.headers());
    println!("Body:\n{}", body);

    Ok(())
}

fn get_weather_from_api(json: &str) -> Option<Report> {
    let document = Html::parse_document(json);

}