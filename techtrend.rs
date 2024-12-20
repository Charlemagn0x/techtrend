use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::env;
use tokio_postgres::{Client, NoTls, Error};

struct AppConfig {
    db_uri: String,
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    
    let config = AppConfig {
        db_uri: env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env"),
    };

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(config.clone()))
            .route("/collect", web::get().to(collect_data))
            .route("/trends", web::get().to(get_trends))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

#[derive(Deserialize)]
struct ApiResponse {
    items: Vec<ApiItem>,
}

#[derive(Deserialize)]
struct ApiItem {
    name: String,
    popularity: u32,
}

async fn collect_data(config: web::Data<AppConfig>) -> impl Responder {
    let res = fetch_data().await;
    let client = connect_to_db(&config.db_uri).await.expect("Failed to connect to database");
    // Improved batch insertion
    insert_data(&client, &res.items).await;
    HttpResponse::Ok().body("Data collected successfully")
}

async fn get_trends(config: web::Data<AppConfig>) -> impl Responder {
    let client = connect_to_db(&config.db_uri).await.expect("Failed to connect to database");
    let trends = fetch_trends(&client).await;
    HttpResponse::Ok().json(trends)
}

async fn connect_to_db(db_uri: &str) -> Result<Client, Error> {
    let (client, connection) = tokio_postgres::connect(db_uri, NoTls).await?;
    
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    Ok(client)
}

async fn fetch_data() -> ApiResponse {
    reqwest::get("https://api.example.com/data")
        .await
        .expect("Failed to fetch data")
        .json::<ApiResponse>()
        .await
        .expect("Failed to parse data")
}

// Optimized for batch insertion
async fn insert_data(client: &Client, items: &Vec<ApiItem>) {
    let transaction = client.transaction().await.expect("Failed to create a database transaction");
    // Preparing the statement outside the loop for efficiency
    let statement = transaction.prepare("INSERT INTO trends (name, popularity) VALUES ($1, $2)").await.expect("Failed to prepare statement");
    for item in items {
        transaction.execute(&statement, &[&item.name, &item.popularity]).await.expect("Failed to insert data");
    }
    // Commit the transaction after all insertions
    transaction.commit().await.expect("Failed to commit transaction");
}

async fn fetch_trends(client: &Client) -> Vec<serde_json::Value> {
    let rows = client.query("SELECT name, popularity FROM trends ORDER BY popularity DESC", &[])
        .await
        .expect("Failed to retrieve data");

    let mut trends = Vec::new();
    
    for row in rows {
        let trend = json!({
            "name": row.get(0),
            "popularity": row.get(1),
        });
        trends.push(trend);
    }
    trends
}