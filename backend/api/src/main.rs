mod config;
mod errors;
mod model;
mod services;
mod handlers;
mod server;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    server::start().await
}
