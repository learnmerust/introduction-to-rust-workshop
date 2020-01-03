use actix_web::{web, App, HttpServer};
use std::sync::Mutex;
use reqwest;

const INTRO: &str = "Rust Server says 👋";
const EXCHANGE_API: &str = "https://api.exchangerate-api.com/v4/latest/CAD";

struct AppState {
    intro: String,
    counter: Mutex<i64>,
}

fn number(number: web::Path<i64>, data: web::Data<AppState>) -> String {
    let mut counter = data.counter.lock().unwrap();
    *counter +=1;

    println!("Request Number: {} :: Path: {}", counter, number);

    let resp: String = reqwest::get(EXCHANGE_API).unwrap()
        .text().unwrap_or("Bad Respoinse".to_string());

    format!("{:#?}", resp)
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
    }).bind("127.0.0.1:8099").unwrap().run().unwrap();
}
