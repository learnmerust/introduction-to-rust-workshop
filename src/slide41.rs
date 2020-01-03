use actix_web::{web, App, HttpResponse, HttpServer};
use failure::Error;
use reqwest;
use std::sync::Mutex;

use askama::Template; // add askama = "=0.8" to Cargo.toml

#[derive(Template)]
#[template(path = "exchange.html")]
struct IndexTemplate<'a> {
    response: &'a str,
}

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

fn number(number: web::Path<f32>, data: web::Data<AppState>) -> Result<HttpResponse, Error> {
    let mut counter = data.counter.lock().unwrap();
    *counter += 1;

    let response = match make_request() {
        Ok(resp) => {
            let requested = number.into_inner();
            let total = resp.rates["USD"] * requested;
            format!(
                "The exchange rate of {} CAD dollars to USD is {} on {}.",
                requested, total, resp.date
            )
        }
        Err(e) => e.to_string(),
    };

    let template = IndexTemplate {
        response: &response,
    }
    .render()
    .unwrap();

    Ok(HttpResponse::Ok().content_type("text/html").body(template))
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
