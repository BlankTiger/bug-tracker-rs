mod auth;
mod routes;
mod utils;
use actix_files::Files;
use actix_files as fs;
use actix_web::{get, web, App, HttpServer, Responder, Result};
use actix_web_lab::middleware::from_fn;
use auth::login::login;
use auth::manage::reject_not_authenticated;
use rand::prelude::*;
use std::path::PathBuf;
use routes::login::login_form;

#[get("/random/{min}-{max}")]
async fn random(args: web::Path<(i32, i32)>) -> impl Responder {
    let (min, max) = args.into_inner();
    let number = rand::thread_rng().gen_range(min..max);
    format!("Your random number is: {}", number)
}

async fn single_page_app() -> Result<fs::NamedFile> {
    let path: PathBuf = PathBuf::from("../yew-frontend/dist/index.html");
    Ok(fs::NamedFile::open(path)?)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    const ACTIX_PORT: &str = std::env!("ACTIX_PORT");

    HttpServer::new(|| {
        App::new()
            .service(
                web::scope("/")
                    .wrap(from_fn(reject_not_authenticated))
                    .service(random)
                    .route("/dashboard", web::get().to(single_page_app))
                    .service(Files::new("/", "../yew-frontend/dist").index_file("index.html")),
            )
            .route("/login", web::get().to(login_form))
            .route("/login", web::post().to(login))
            //.route("/register", web::get().to(register_form))) 
            //.route("/register", web::post().to(register))) 
            //.route("/logout", web::get().to(logout))")
    })
    .bind(("127.0.0.1", ACTIX_PORT.parse::<u16>().unwrap_or(8080)))?
    .run()
    .await
}
