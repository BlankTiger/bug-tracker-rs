use actix_files::Files;
use actix_web::{get, web, App, HttpServer, Responder};
use rand::prelude::*;

#[get("/hello")]
async fn hello() -> impl Responder {
    "Hello, World!"
}

#[get("/hello/{name}")]
async fn hello_msg(name: web::Path<String>) -> impl Responder {
    format!("Hello, {}! Your name has a length of: {}", name, name.len())
}

#[get("/random/{min}-{max}")]
async fn random(args: web::Path<(i32, i32)>) -> impl Responder {
    let (min, max) = args.into_inner();
    let number = rand::thread_rng().gen_range(min..max);
    format!("Your random number is: {}", number)
}

#[get("/file/{filename}")]
async fn file(filename: web::Path<String>) -> impl Responder {
    let file_contents = std::fs::read_to_string(filename.into_inner())
        .unwrap_or_else(|_| "File not found".to_string());
    dbg!(&file_contents);
    file_contents
}

#[get("/current_path")]
async fn current_path() -> impl Responder {
    let current_path = match std::env::current_dir() {
        Ok(path) => path.to_str().unwrap_or("Couldnt convert").to_string(),
        Err(e) => e.to_string(),
    };
    current_path
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    const ACTIX_PORT: &str = std::env!("ACTIX_PORT");
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(hello_msg)
            .service(random)
            .service(file)
            .service(current_path)
            .service(Files::new("/", "../yew-frontend/dist").index_file("index.html"))
    })
    .bind(("127.0.0.1", ACTIX_PORT.parse::<u16>().unwrap_or(8080)))?
    .run()
    .await
}
