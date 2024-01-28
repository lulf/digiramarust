use actix_web::{get, web, App, HttpServer, Responder};
use std::sync::Mutex;

struct AppStateWithCounter {
    counter: Mutex<i32>, // <- Mutex is necessary to mutate safely across threads
}

#[get("/")]
async fn index(data: web::Data<AppStateWithCounter>) -> String {
    let mut counter = data.counter.lock().unwrap(); // <- get counter's MutexGuard
    *counter += 1; // <- access counter inside MutexGuard

    format!("Request number: {counter}") // <- response with count
}

#[get("/{name}")]
async fn hello(name: web::Path<String>) -> impl Responder {
    format!("Hello {}!", &name)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let counter = web::Data::new(AppStateWithCounter {
        counter: Mutex::new(0),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(counter.clone())
            .service(index)
            .service(hello)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
