use crate::database::common::setup_pool;
use crate::init::configure_webapp;
use crate::utils::{create_mailer, create_reloader, AppState};
use actix_cors::Cors;
use actix_identity::IdentityMiddleware;
use actix_multipart::form::MultipartFormConfig;
use actix_session::config::PersistentSession;
use actix_session::{storage::CookieSessionStore, SessionMiddleware};
use actix_web::http::header;
use actix_web::middleware::Logger;
use actix_web::web::{FormConfig, PayloadConfig};
use actix_web::{cookie::Key, App, HttpServer};
use anyhow::anyhow;
use env_logger::Env;
use log::{info, warn};
use std::env;
use std::sync::Arc;

mod database;
mod error;
mod forms;
mod handlers;
mod init;
mod templates;
mod utils;

const DEFAULT_HOSTNAME: &str = "localhost";
const DEFAULT_PORT: &str = "8000";
const SECS_IN_WEEK: i64 = 60 * 60 * 24 * 7;
const PAYLOAD_LIMIT: usize = 16 * 1024 * 1024 * 1024; // 16GiB

const FORM_LIMIT: usize = 16 * 1024 * 1024; // 16MiB
const MIN_PASS_LEN: usize = 6;

const THUMBNAIL_SIZE: u32 = 600;
const IMAGE_SIZE: u32 = 2000;

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    env::set_var("TMPDIR", "./media");
    let _dir = env::temp_dir();

    let pool = setup_pool(10_u32).await?;
    let jinja = Arc::new(create_reloader("templates".to_owned()));
    let mailer = Arc::new(create_mailer().map_err(|e| anyhow!(e.message))?);

    let app_state = AppState::new(jinja.clone(), mailer.clone());

    let host = parse_host();
    let host2 = host.clone();

    let key = Key::from(
        &env::var("COOKIE_SESSION_KEY")
            .unwrap_or_default()
            .bytes()
            .collect::<Vec<u8>>(),
    );
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let use_secure_cookie = env::var("USE_SECURE_COOKIE")
        .unwrap_or("false".to_string())
        .parse::<bool>()?;
    info!("USE_SECURE_COOKIE: {}", use_secure_cookie);

    if let Err(e) = dotenvy::dotenv() {
        warn!("failed loading .env file: {e}");
    };
    info!("starting server on {host}");

    HttpServer::new(move || {
        App::new()
            .app_data(
                MultipartFormConfig::default()
                    .total_limit(PAYLOAD_LIMIT)
                    .memory_limit(PAYLOAD_LIMIT),
            )
            .app_data(FormConfig::default().limit(FORM_LIMIT))
            .app_data(PayloadConfig::new(PAYLOAD_LIMIT))
            .wrap(IdentityMiddleware::default())
            .wrap(
                SessionMiddleware::builder(CookieSessionStore::default(), key.clone())
                    .cookie_secure(use_secure_cookie)
                    .session_lifecycle(
                        PersistentSession::default()
                            .session_ttl(actix_web::cookie::time::Duration::seconds(SECS_IN_WEEK)),
                    )
                    .build(),
            )
            .wrap(
                Cors::default()
                    .allowed_origin(format!("http://{}", host).as_str())
                    .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "PATCH"])
                    .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                    .allowed_header(header::CONTENT_TYPE)
                    .supports_credentials()
                    .max_age(3600),
            )
            .wrap(Logger::default())
            .configure(configure_webapp(&pool, app_state.clone()))
    })
    .bind(host2)?
    .run()
    .await?;
    Ok(())
}

fn parse_host() -> String {
    let hostname = env::var("HOSTNAME").unwrap_or(DEFAULT_HOSTNAME.to_string());
    let port = env::var("PORT").unwrap_or(DEFAULT_PORT.to_string());
    format!("{hostname}:{port}")
}
