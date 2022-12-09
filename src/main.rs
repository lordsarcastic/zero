use actix_web::{HttpRequest, Responder};
use std::net::TcpListener;
use zero::run;

async fn _greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {name}")
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8000").expect("Failed to bind to a random port");
    run(listener)?.await
}
