use std::net::TcpListener;

use sqlx::PgPool;
use uuid::Uuid;

pub struct TestApp {
    pub address: String,
    pub connection_pool: PgPool
}
pub async fn spawn_app() -> TestApp {
    let mut configuration = zero::configuration::get_configuration()
        .expect("Failed to read configuration");
    configuration.database.name = Uuid::new_v4().to_string();
    let connection_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to initialize connection pool");
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind to a random port");
    let port = listener.local_addr().unwrap().port();
    let server = zero::run(listener, connection_pool.clone()).expect("Failed to bind address");
    let _ = tokio::spawn(server);
    TestApp {
        address: format!("http://127.0.0.1:{}", port),
        connection_pool: connection_pool
    }
}