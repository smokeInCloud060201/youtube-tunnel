mod config;
mod model;
mod services;
mod server;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    server::start().await
}
