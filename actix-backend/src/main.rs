use actix_files::Files;
use actix_web::{get, web, App, HttpServer, Responder};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use rand::prelude::*;

#[get("/random/{min}-{max}")]
async fn random(args: web::Path<(i32, i32)>) -> impl Responder {
    let (min, max) = args.into_inner();
    let number = rand::thread_rng().gen_range(min..max);
    format!("Your random number is: {}", number)
}

#[get("/current_path")]
async fn current_path() -> impl Responder {
    let path = std::env::current_dir().unwrap();
    format!("Current path is: {}", path.display())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    const ACTIX_PORT: &str = std::env!("ACTIX_PORT");

    // configure ssl
    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder.set_private_key_file("key.pem", SslFiletype::PEM).unwrap();
    builder.set_certificate_chain_file("cert.pem").unwrap();

    HttpServer::new(|| {
        App::new()
            .service(random)
            .service(current_path)
            .service(Files::new("/", "../yew-frontend/dist").index_file("index.html"))
    })
    .bind_openssl(("127.0.0.1", ACTIX_PORT.parse::<u16>().unwrap_or(8080)), builder)?
    .run()
    .await
}
