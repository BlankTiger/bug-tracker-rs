mod auth;
mod configurations;
mod routes;
mod utils;
use crate::configurations::DatabaseSettings;
use actix_files as fs;
use actix_files::Files;
use actix_session::{storage::RedisSessionStore, SessionMiddleware};
use actix_web::cookie::Key;
use actix_web::dev::Server;
use actix_web::{web, App, HttpServer, Result};
use actix_web_lab::middleware::from_fn;
use auth::login::login;
use auth::manage::reject_not_authenticated;
use routes::login::login_form;
use secrecy::{ExposeSecret, Secret};
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};
use std::net::TcpListener;
use std::path::PathBuf;

async fn single_page_app() -> Result<fs::NamedFile> {
    let path: PathBuf = PathBuf::from("../yew-frontend/dist/index.html");
    Ok(fs::NamedFile::open(path)?)
}

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    let app = server().await?;
    app.await?;
    Ok(())
}

async fn server() -> std::result::Result<Server, anyhow::Error> {
    const ACTIX_PORT: &str = std::env!("ACTIX_PORT");

    let redis_store = RedisSessionStore::new("redis://127.0.0.1:6379/").await?;
    let secret_value = Secret::new(vec![0; 128]);
    let secret_key = Key::from(secret_value.expose_secret());

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
        .password(database_settings.password.expose_secret())
        .ssl_mode(sqlx::postgres::PgSslMode::Disable);

    let db_pool = PgPoolOptions::new()
        .acquire_timeout(std::time::Duration::from_secs(2))
        .connect_lazy_with(pg_options.database(&database_settings.database_name));

    let db_pool = web::Data::new(db_pool);

    let listener = TcpListener::bind(&format!(
        "127.0.0.1:{}",
        ACTIX_PORT.parse::<u16>().unwrap_or(8080)
    ))?;

    let server = HttpServer::new(move || {
        App::new()
            .wrap(SessionMiddleware::new(
                redis_store.clone(),
                secret_key.clone(),
            ))
            .app_data(db_pool.clone())
            .route("/login", web::get().to(login_form))
            .route("/login", web::post().to(login))
            .service(
                web::scope("/dashboard")
                    .wrap(from_fn(reject_not_authenticated))
                    .route("/home", web::get().to(single_page_app)),
            )
            .service(Files::new("/", "../yew-frontend/dist").index_file("index.html"))
        //.route("/register", web::get().to(register_form)))
        //.route("/register", web::post().to(register)))
        //.route("/logout", web::get().to(logout))")
    })
    .listen(listener)?
    .run();
    Ok(server)
}
