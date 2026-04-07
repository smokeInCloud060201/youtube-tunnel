mod pipeline;
mod server;
mod youtube;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    server::start().await
}
