use actix_web::{web, App, HttpServer, client};

const INTRO: &str = "Rust Server says 👋";

struct AppState {
    intro: String,
}

fn index(data: web::Data<AppState>) -> String {
    format!("{}", data.intro);
}

fn main() {
    let app_state = web::Data::new(AppState {
        intro: INTRO,
    });

    HttpServer::new(move || {
        App::new()
            .register_data(app_state.clone())
            .route("/", web::get().to(index))
    }).bind("127.0.0.1:8099").unwrap().run().unwrap();
}
