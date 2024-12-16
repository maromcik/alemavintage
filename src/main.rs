use crate::database::common::setup_pool;
use crate::init::configure_webapp;
use actix_cors::Cors;
use actix_identity::IdentityMiddleware;
use actix_multipart::form::MultipartFormConfig;
use actix_session::config::PersistentSession;
use actix_session::{storage::CookieSessionStore, SessionMiddleware};
use actix_web::http::header;
use actix_web::middleware::Logger;
use actix_web::web::PayloadConfig;
use actix_web::{cookie::Key, App, HttpServer};
use env_logger::Env;
use log::{info, warn};
use minijinja::{path_loader, Environment, Error, Value};
use minijinja_autoreload::AutoReloader;
use std::env;
use std::sync::Arc;

mod database;
mod error;
mod forms;
mod handlers;
mod init;
mod templates;

const DEFAULT_HOSTNAME: &str = "localhost";
const DEFAULT_PORT: &str = "8000";
const SECS_IN_WEEK: i64 = 60 * 60 * 24 * 7;
const PAYLOAD_LIMIT: usize = 16 * 1024 * 1024 * 1024; // 16GiB
const MIN_PASS_LEN: usize = 6;

pub struct AppState {
    pub jinja: Arc<AutoReloader>,
}

impl AppState {
    pub fn new(jinja: Arc<AutoReloader>) -> Self {
        AppState { jinja }
    }
}

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    // We are forced to change the TMP dir because TmpFile that is used in Actix multipart stores uploaded files in the /tmp dir.
    // by default, and I was not able to alter the default configuration,
    // then, after calling function persist, it uses the rename(2) syscall to unlink the file from /tmp and link
    // it to another folder. However, this syscall fails on an attempt to move the file across file system boundaries.
    // On many distros /tmp uses tmpfs and is mounted separately. Also, we are deploying in Kubernetes, and while the /tmp
    // dir is not mounted separately, we use persistent volume claims to take advantage of the large NFS storage,
    // so the target file path is on a different FS as well.
    env::set_var("TMPDIR", "./media");
    let _dir = env::temp_dir();

    let pool = setup_pool(10_u32).await?;

    let reloader = AutoReloader::new(move |notifier| {
        let template_path = "templates";
        let mut env = Environment::new();
        env.set_loader(path_loader(&template_path));
        notifier.set_fast_reload(true);
        notifier.watch_path(template_path, true);
        Ok(env)
    });

    // let mut env = Environment::new();
    // env.set_loader(path_loader("templates"));
    let jinja = Arc::new(reloader);

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
            .configure(configure_webapp(&pool, jinja.clone()))
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
