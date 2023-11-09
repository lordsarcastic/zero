use actix_web::{HttpRequest, Responder};
use std::net::TcpListener;
use zero::{startup::run, configuration::get_configuration};

async fn _greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {name}")
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let config = get_configuration()
        .expect("Failed to read configuration");
    let address = format!("127.0.0.1:{}", config.application_port);
    let listener = TcpListener::bind(address)
        .expect("Failed to bind to a random port");
    run(listener)?.await
}
