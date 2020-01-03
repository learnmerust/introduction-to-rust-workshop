use failure::Error;
use actix_web::{web, App, HttpServer};
use reqwest;
use std::sync::Mutex;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
struct ExchangeResponse {
    base: String,
    date: String,
    time_last_updated: i32,
    rates: HashMap<String, f32>,
}

const INTRO: &str = "Rust Server says 👋";
const EXCHANGE_API: &str = "https://api.exchangerate-api.com/v4/latest/CAD";

struct AppState {
    intro: String,
    counter: Mutex<i64>,
}

fn make_request() -> Result<ExchangeResponse, Error> {
    let req: String = reqwest::get(EXCHANGE_API)?.text()?;
    let resp: ExchangeResponse = serde_json::from_str(&req)?;
    Ok(resp)
}

fn number(number: web::Path<i64>, data: web::Data<AppState>) -> String {
    let mut counter = data.counter.lock().unwrap();
    *counter += 1;

    println!("Request Number: {} :: Path: {}", counter, number);

    match make_request() {
        Ok(resp) => format!(
            "The exchange rate of 1 {} to USD is {} on {}.",
            resp.base, resp.rates["USD"], resp.date
        ),
        Err(e) => e.to_string(),
    }
}

fn index(data: web::Data<AppState>) -> String {
    format!("{}", data.intro)
}

fn main() {
    let app_state = web::Data::new(AppState {
        intro: INTRO.to_string(),
        counter: Mutex::new(0),
    });

    HttpServer::new(move || {
        App::new()
            .register_data(app_state.clone())
            .route("/", web::get().to(index))
            .route("/{number}", web::get().to(number))
    })
    .bind("127.0.0.1:8099")
    .unwrap()
    .run()
    .unwrap();
}
