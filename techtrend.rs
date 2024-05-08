use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;
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
            .data(config.clone())
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
    let res = reqwest::get("https://api.example.com/data")
        .await
        .expect("Failed to fetch data")
        .json::<ApiResponse>()
        .await
        .expect("Failed to parse data");
    
    let client = connect_to_db(&config.db_uri).await.expect("Failed to connect to database");

    for item in res.items {
        client.execute("INSERT INTO trends (name, popularity) VALUES ($1, $2)", &[&item.name, &item.popularity])
            .await
            .expect("Failed to insert data");
    }

    HttpResponse::Ok().body("Data collected successfully")
}

async fn get_trends(config: web::Data<AppConfig>) -> impl Responder {
    let client = connect_to_db(&config.db_uri).await.expect("Failed to connect to database");
    
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