use actix_backend::server_init::server;

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    server().await?.await?;
    Ok(())
}
