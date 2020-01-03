use actix_web::{web, App, HttpServer};
use std::sync::Mutex;
use reqwest;

const INTRO: &str = "Rust Server says 👋";
const EXCHANGE_API: &str = ""

struct AppState {
    intro: String,
    counter: Mutex<i64>,
}

fn number(number: web::Path<i64>, data: web::Data<AppState>) -> String {
    let thread = std::thread::current().id();
    println!("thread {:?}", thread);

    let mut counter = data.counter.lock().unwrap();
    *counter +=1;

    format!("Request Number: {} :: Path: {}", counter, number)
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
