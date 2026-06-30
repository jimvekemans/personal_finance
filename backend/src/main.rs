use axum::{extract::State, routing::post, Json, Router};
use serde::Deserialize;
use sqlx::postgres::{PgPool, PgPoolOptions};
use std::env;
use std::sync::Arc;
use bigdecimal::BigDecimal; // <-- ADD THIS

#[derive(Deserialize)]
struct ExpensePayload {
    item: String,
    cost: BigDecimal, // <-- CHANGE f64 to BigDecimal
    category: String,
    who_paid: String,
}

// 1. Create a struct to hold our shared database pool
struct AppState {
    db: PgPool,
}

#[tokio::main]
async fn main() {
    let db_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://admin:secret_pass@db:5432/finance_db".to_string());

    // 2. Initialize the connection pool ONCE at application startup
    let pool = PgPoolOptions::new()
        .max_connections(50)
        .connect(&db_url)
        .await
        .expect("Failed to connect to Postgres");

    // Wrap the state in an Arc (Atomic Reference Counted) to safely share across threads
    let shared_state = Arc::new(AppState { db: pool });

    // Inject the state into Axum
    let app = Router::new()
        .route("/api/expenses", post(add_expense))
        .with_state(shared_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    println!("Listening on 0.0.0.0:8080");
    
    // This call blocks indefinitely
    axum::serve(listener, app).await.unwrap();
}

// 3. Inject the shared state into the handler
async fn add_expense(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<ExpensePayload>,
) -> Result<&'static str, axum::http::StatusCode> {
    
    // 4. Use sqlx::query! for cached, injected-safe prepared statements
    let result = sqlx::query!(
        r#"
        INSERT INTO expenses (item, cost, category, who_paid) 
        VALUES ($1, $2, $3, $4)
        "#,
        payload.item,
        payload.cost,
        payload.category,
        payload.who_paid
    )
    .execute(&state.db)
    .await;

    match result {
        Ok(_) => Ok("Expense added"),
        Err(e) => {
            eprintln!("Database error: {}", e);
            Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}