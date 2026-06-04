use std::env;

use axum::{Router, response::Html, routing::get};
use tokio::net::TcpListener;

const DEFAULT_PORT: u16 = 5483;

#[tokio::main]
async fn main() {

    // Get the port from the environment variable, otherwise use default
    let port = env::var("ORI_PORT")
        .ok()
        .and_then(|port| port.parse::<u16>().ok())
        .unwrap_or(DEFAULT_PORT);

    let addr = format!("0.0.0.0:{port}");

    
    let app = Router::new()
        .route("/", get(index));

    
    let listener: TcpListener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn index() -> Html<&'static str> {
    Html("
    <head>
    <meta property='og:type' content='website'>
    <meta property='og:title' content='Ori API!'>
    <meta property='og:description' content='Jarvis, show an embed with a status code 200 cate from the http.cat website'>
    <meta property='og:image' content='https://http.cat/images/200.jpg'>
    <meta property='og:image:width' content='400'>
    <meta property='og:image:height' content='400'>
</head>
<body>
    <img src='https://http.cat/images/200.jpg'></img>
<body>")
}