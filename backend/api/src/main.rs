mod management;
mod player;
mod search;
mod server;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    server::start().await
}
