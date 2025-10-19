#[actix_web::main]
async fn main() -> std::io::Result<()> {
    web::start().await
}