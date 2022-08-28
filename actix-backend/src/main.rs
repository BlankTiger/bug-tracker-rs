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
use actix_web::http::header::LOCATION;
use actix_web::{web, App, HttpResponse, HttpServer, Result};
use actix_web_lab::middleware::from_fn;
use auth::login::{login, logout};
use auth::manage::reject_not_authenticated;
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use routes::login::login_form;
use secrecy::{ExposeSecret, Secret};
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};
use std::net::TcpListener;
use std::path::PathBuf;

async fn single_page_app() -> Result<fs::NamedFile> {
    let path: PathBuf = PathBuf::from("../yew-frontend/dist/index.html");
    Ok(fs::NamedFile::open(path)?)
}

async fn redirect_to_login() -> HttpResponse {
    HttpResponse::SeeOther()
        .insert_header((LOCATION, "/login"))
        .finish()
}

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    server().await?.await?;
    Ok(())
}

async fn server() -> std::result::Result<Server, anyhow::Error> {
    const ACTIX_PORT: &str = std::env!("ACTIX_PORT");
    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder
        .set_private_key_file("key.pem", SslFiletype::PEM)
        .unwrap();
    builder.set_certificate_chain_file("cert.pem").unwrap();

    let redis_store = RedisSessionStore::new("redis://127.0.0.1:6379/").await?;
    let secret_key = Key::generate();

    let database_settings = DatabaseSettings {
        host: "localhost".to_string(),
        port: 5432,
        username: "postgres".to_string(),
        password: Secret::from("postgres".to_string()),
        database_name: "users".to_string(),
        require_ssl: true,
    };

    let pg_options = PgConnectOptions::new()
        .host(&database_settings.host)
        .port(database_settings.port)
        .username(&database_settings.username)
        .password(database_settings.password.expose_secret())
        .ssl_mode(sqlx::postgres::PgSslMode::Prefer);

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
            .route("/", web::get().to(redirect_to_login))
            .service(
                web::scope("/dashboard")
                    .wrap(from_fn(reject_not_authenticated))
                    .route("/home", web::get().to(single_page_app))
                    .route("/logout", web::get().to(logout)),
            )
            .service(Files::new("/", "../yew-frontend/dist"))
        // TODO: add register
        //.route("/register", web::get().to(register_form)))
        //.route("/register", web::post().to(register)))
    })
    .listen_openssl(listener, builder)?
    .run();
    Ok(server)
}
