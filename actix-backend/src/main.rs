mod auth;
mod routes;
mod utils;
mod configurations;
use actix_files as fs;
use actix_files::Files;
use actix_web::{web, App, HttpServer, Result};
use actix_web_lab::middleware::from_fn;
use auth::login::login;
use auth::manage::reject_not_authenticated;
use routes::login::login_form;
use secrecy::{Secret, ExposeSecret};
use sqlx::postgres::{PgPoolOptions, PgConnectOptions};
use std::path::PathBuf;
use crate::configurations::DatabaseSettings;

async fn single_page_app() -> Result<fs::NamedFile> {
    let path: PathBuf = PathBuf::from("../yew-frontend/dist/index.html");
    Ok(fs::NamedFile::open(path)?)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    //TODO: add configuration with app_data
    const ACTIX_PORT: &str = std::env!("ACTIX_PORT");

    let database_settings = DatabaseSettings {
        host: "localhost".to_string(),
        port: 5432,
        username: "postgres".to_string(),
        password: Secret::from("postgres".to_string()),
        database_name: "users".to_string(),
        require_ssl: false,
    };

    let pg_options = PgConnectOptions::new()
        .host(&database_settings.host)
        .port(database_settings.port)
        .username(&database_settings.username)
        .password(&database_settings.password.expose_secret())
        .ssl_mode(sqlx::postgres::PgSslMode::Disable);

    let db_pool = PgPoolOptions::new()
        .acquire_timeout(std::time::Duration::from_secs(2))
        .connect_lazy_with(pg_options);

    let db_pool = web::Data::new(db_pool);

    HttpServer::new(move || {
        App::new()
            .route("/login", web::get().to(login_form))
            .route("/login", web::post().to(login))
            .service(
                web::scope("/")
                    .wrap(from_fn(reject_not_authenticated))
                    .route("/dashboard", web::get().to(single_page_app))
                    .service(Files::new("/", "../yew-frontend/dist").index_file("index.html")),
            )
            .app_data(db_pool.clone())
        //.route("/register", web::get().to(register_form)))
        //.route("/register", web::post().to(register)))
        //.route("/logout", web::get().to(logout))")
    })
    .bind(("127.0.0.1", ACTIX_PORT.parse::<u16>().unwrap_or(8080)))?
    .run()
    .await
}
