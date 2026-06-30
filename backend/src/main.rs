// /home/jim/Documents/finance_tracker/backend/src/main.rs
use adbc_core::{options::AdbcVersion, Connection, Database, Optionable, Statement};
use adbc_driver_manager::ManagedDriver;
use axum::{routing::post, Json, Router};
use serde::Deserialize;
use std::env;

#[derive(Deserialize)]
struct ExpensePayload {
    item: String,
    cost: f64,
    category: String,
    who_paid: String,
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/api/expenses", post(add_expense));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    println!("Listening on 0.0.0.0:8080");
    axum::serve(listener, app).await.unwrap();
}

async fn add_expense(Json(payload): Json<ExpensePayload>) -> Result<&'static str, axum::http::StatusCode> {
    // Load the ADBC Driver dynamically
    let mut driver = ManagedDriver::load_dynamic_from_filename(
        "/usr/lib/libadbc_driver_postgresql.so",
        None,
        AdbcVersion::V1_0_0,
    ).map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    let db_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://admin:secret_pass@db:5432/finance_db".to_string());

    let mut database = driver
        .new_database()
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
        
    database
        .set_option("uri", &db_url)
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
        
    database
        .init()
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    let mut connection = database
        .new_connection()
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
        
    connection
        .init()
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    let mut statement = connection
        .new_statement()
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    let sql = format!(
        "INSERT INTO expenses (item, cost, category, who_paid) VALUES ('{}', {}, '{}', '{}')",
        payload.item.replace("'", "''"), // Basic escaping
        payload.cost,
        payload.category.replace("'", "''"),
        payload.who_paid.replace("'", "''")
    );

    statement
        .set_sql_query(&sql)
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
        
    statement
        .execute()
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok("Expense added")
}