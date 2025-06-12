use crate::app::App;
use crate::env;
use axum;
use dioxus::prelude::{DioxusRouterExt, ServeConfigBuilder};
use log::{info, LevelFilter};
use simple_logger::SimpleLogger;
use std::net::SocketAddr;

pub async fn run_server() {
    SimpleLogger::new()
        .with_level(LevelFilter::Warn)
        .with_module_level("credit", LevelFilter::Info)
        .init()
        .unwrap();

    info!("Starting server");

    let app = axum::Router::new()
        .serve_dioxus_application(ServeConfigBuilder::default(), App)
        .layer(axum::middleware::from_fn(log_requests));

    let router = app.into_make_service();

    let ip = env::ip_address();
    let port = env::port();

    let address = SocketAddr::new(ip, port);
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();

    axum::serve(listener, router).await.unwrap();

    info!("Started server at {}:{}", ip, port)
}

async fn log_requests(
    request: axum::extract::Request,
    next: axum::middleware::Next,
) -> axum::response::Response {
    let method = request.method().to_string();
    let uri = request.uri().to_string();

    let response = next.run(request).await;

    let status = response.status().as_u16();
    info!("{} {} - {}", method, uri, status);

    response
}
