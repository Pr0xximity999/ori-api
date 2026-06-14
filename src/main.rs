use std::env;
use axum::{
    Router, extract::Path, http::StatusCode, response::{Html, IntoResponse, Json}, routing::get};
use rusqlite::Connection;
use serde_json::json;
use tokio::net::TcpListener;

use crate::models::Sentence;
const DEFAULT_PORT: u16 = 5483;

mod models;

#[tokio::main]
async fn main() {

    // Get the port from the environment variable, otherwise use default
    let port = env::var("ORI_PORT")
        .ok()
        .and_then(|port| port.parse::<u16>().ok())
        .unwrap_or(DEFAULT_PORT);

    let addr = format!("0.0.0.0:{port}");
    
    let app = Router::new()
        .route("/", get(code(Path(404)).await))
        .route("/code/{code}", get(code))
        .route("/zin/all",get(sentence_all))
        .fallback(code(Path(404)).await);

    
    let listener: TcpListener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn code(Path(code): Path<u16>) -> (StatusCode, Html<String>)
{
    let hypertext = format!("
    <head>
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

async fn sentence_all() -> Result<impl IntoResponse, Json<String>>
{
    // Estrablish connection
    let connection = Connection::open("batadase.db").unwrap();

    // Prepare query
    let query = "SELECT * FROM Sentences";
    let mut statement = connection.prepare(query).unwrap();

    // Query database
    let rows = statement.query_map([], |row| {
        Ok(Sentence {
            sentence: row.get(0)?
        })
    }).unwrap();

    let mut sentences = Vec::new();
    sentences.push(Sentence {sentence: "You're not supposed to be here".to_owned() });
    for sentence in rows {
        sentences.push(sentence.unwrap());
    }

    
    // Return data
    Ok(Json(json!({"sentences" : sentences})))
}