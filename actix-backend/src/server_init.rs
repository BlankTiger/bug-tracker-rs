use crate::auth::manage::reject_not_authenticated;
use crate::configurations::DatabaseSettings;
use crate::routes::get::app::single_page_app;
use crate::routes::get::login::{login_form, redirect_to_login};
use crate::routes::get::projects::get_projects;
use crate::routes::post::login::{login, logout};
use actix_files::Files;
use actix_session::{storage::RedisSessionStore, SessionMiddleware};
use actix_web::cookie::Key;
use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use actix_web_lab::middleware::from_fn;
use openssl::ssl::{SslAcceptor, SslAcceptorBuilder, SslFiletype, SslMethod};
use secrecy::{ExposeSecret, Secret};
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};
use std::net::TcpListener;

fn ssl_builder() -> Result<SslAcceptorBuilder, anyhow::Error> {
    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls())?;
    builder.set_private_key_file("key.pem", SslFiletype::PEM)?;
    builder.set_certificate_chain_file("cert.pem")?;
    Ok(builder)
}

fn listener(actix_port: u16) -> Result<TcpListener, anyhow::Error> {
    let listener = TcpListener::bind(&format!("127.0.0.1:{}", actix_port))?;
    Ok(listener)
}

/// TODO: Separate ssl and settings out of server
pub async fn server() -> std::result::Result<Server, anyhow::Error> {
    let actix_port: u16 = std::env!("ACTIX_PORT").parse::<u16>().unwrap_or(8080);

    let listener = listener(actix_port)?;
    let ssl_builder = ssl_builder()?;
    let redis_store = RedisSessionStore::new("redis://127.0.0.1:6379/").await?;
    let secret_key = Key::generate();

    let database_settings = DatabaseSettings {
        host: "localhost".to_string(),
        port: 5432,
        username: "postgres".to_string(),
        password: Secret::from("postgres".to_string()),
        database_name: "bug_tracker".to_string(),
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
            .service(
                web::scope("/api")
                    .wrap(from_fn(reject_not_authenticated))
                    .route("/projects/get_projects", web::get().to(get_projects)),
            )
            .service(Files::new("/", "../yew-frontend/dist"))
        // TODO: add register
        //.route("/register", web::get().to(register_form)))
        //.route("/register", web::post().to(register)))
    })
    .listen_openssl(listener, ssl_builder)?
    .run();
    Ok(server)
}
