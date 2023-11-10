use std::net::TcpListener;

use sqlx::{Connection, Executor, PgConnection, PgPool};
use uuid::Uuid;
use zero::configuration::DatabaseSettings;

pub struct TestApp {
    pub address: String,
    pub connection_pool: PgPool,
}

pub async fn spawn_app() -> TestApp {
    let mut configuration =
        zero::configuration::get_configuration().expect("Failed to read configuration");
    configuration.database.name = Uuid::new_v4().to_string();
    let connection_pool = configure_database(&configuration.database).await;
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind to a random port");
    let port = listener.local_addr().unwrap().port();
    let server = zero::run(listener, connection_pool.clone()).expect("Failed to bind address");
    let _ = tokio::spawn(server);
    TestApp {
        address: format!("http://127.0.0.1:{}", port),
        connection_pool: connection_pool,
    }
}

pub async fn configure_database(database_settings: &DatabaseSettings) -> PgPool {
    let mut connection = PgConnection::connect(&database_settings.connection_string_without_db())
        .await
        .expect("Failed to connect to Posgres");
    connection
        .execute(
            format!(
                r#"
            CREATE DATABASE "{}";
            "#,
                &database_settings.name
            )
            .as_str(),
        )
        .await
        .expect("Failed to create test database");
    let connection_pool = PgPool::connect(&database_settings.connection_string())
        .await
        .expect("Failed to initialize connection pool");
    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("Failed to run migrations on test database");
    connection_pool
}
