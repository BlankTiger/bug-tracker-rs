use actix_files::Files;
use actix_web::{get, web, App, HttpServer, Responder};
use rand::prelude::*;

#[get("/random/{min}-{max}")]
async fn random(args: web::Path<(i32, i32)>) -> impl Responder {
    let (min, max) = args.into_inner();
    let number = rand::thread_rng().gen_range(min..max);
    format!("Your random number is: {}", number)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    const ACTIX_PORT: &str = std::env!("ACTIX_PORT");
    HttpServer::new(|| {
        App::new()
            .service(random)
            .service(Files::new("/", "../yew-frontend/dist").index_file("index.html"))
    })
    .bind(("127.0.0.1", ACTIX_PORT.parse::<u16>().unwrap_or(8080)))?
    .run()
    .await
}
