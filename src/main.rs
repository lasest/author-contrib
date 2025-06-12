#[allow(unused_imports)]
use app::App;

mod app;
mod assets;
mod backend;
mod components;
mod env;
mod models;
mod router;
#[cfg(feature = "server")]
mod server;
mod views;

#[cfg(feature = "server")]
use server::run_server;

/// The entry point for the server
#[cfg(feature = "server")]
#[tokio::main]
async fn main() {
    run_server().await;
}

/// The entry point for the client-side application
#[cfg(not(feature = "server"))]
fn main() {
    dioxus::launch(App);
}
