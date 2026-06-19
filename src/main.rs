use std::{env, sync::Arc};
use axum::{
    Router, extract::{Path, State}, http::StatusCode, response::{Html, IntoResponse, Json}, routing::get};
use dotenv::dotenv;
use serde_json::json;
use sqlx::{MySqlPool, mysql::MySqlPoolOptions};
use tokio::net::TcpListener;

use crate::models::Sentence;
const DEFAULT_PORT: u16 = 5483;

// File imports
mod models;

pub struct DBState {
    db: MySqlPool,
}

#[tokio::main]
async fn main() {
    
    // Load env variables
    println!("Loading environment variables");
    dotenv().ok();
    let port = env::var("ORI_PORT")
    .ok()
    .and_then(|port| port.parse::<u16>().ok())
    .unwrap_or(DEFAULT_PORT);
    println!("Running ori api on port: {port}");

    let db_url = env::var("DATABASE_URL").expect("Database url not set");


    //DB connection
    println!("Starting database...");
    let pool = match MySqlPoolOptions::new()
        .max_connections(10)
        .connect(&db_url)
        .await {
        Ok(pool) => {
            println!("Successfully connected to db!");
            pool
        },
        Err(e) => {
            println!("Failed to connect to db: {e}");
            std::process::exit(1);
        }
    }; 
    

    // Load API
    let addr = format!("0.0.0.0:{port}");
    
    let app = Router::new()
        .route("/", get(code(Path(200)).await))
        .route("/code/{code}", get(code))
        .route("/zin/all",get(sentence_all))
        .with_state(Arc::new(DBState { db: pool.clone() }))
        .fallback(code(Path(404)).await);

    
    let listener: TcpListener = tokio::net::TcpListener::bind(addr.clone()).await.unwrap();
    println!("Successfully started listening on {addr}");

    axum::serve(listener, app).await.unwrap();
}

async fn code(Path(code): Path<u16>) -> (StatusCode, Html<String>)
{
    let hypertext = format!("<head>
        <meta property='og:type' content='website'>
        <meta property='og:title' content='Ori API!'>
        <meta property='og:description' content='Jarvis, show an embed with a status code {code} cate from the http.cat website'>
        <meta property='og:image' content='https://http.cat/images/{code}.jpg'>
        <meta property='og:image:width' content='400'>
        <meta property='og:image:height' content='400'>
    </head>
    <img src='https://http.cat/{code}'></img>
    ");
    (StatusCode::from_u16(code).unwrap(), Html(hypertext))
}

async fn sentence_all(State(state) : State<Arc<DBState>>) -> Result<impl IntoResponse, Json<String>>
{

    // Query database
    let mut sentences: Vec<Sentence> = Vec::new();

    match sqlx::query_as!(
            Sentence, 
        "SELECT * FROM sentences"
        )
        .fetch_all(&state.db)
        .await {
            Ok(val) => {
                for sentence in val  {
                    sentences.push(sentence);
                }
            },
            Err(e) => {
                println!("An error occurred while querying database: {e}")
            }
        };
    
    // Return data
    Ok(Json(json!({"sentences" : sentences})))
}