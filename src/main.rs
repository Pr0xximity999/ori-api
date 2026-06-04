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
    Html("<img src='https://http.cat/images/200.jpg'></img>")
}