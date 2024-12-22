use crate::error::AppError;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{AsyncSmtpTransport, Tokio1Executor};
use minijinja::{path_loader, Environment};
use minijinja_autoreload::AutoReloader;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub jinja: Arc<AutoReloader>,
    pub mailer: Arc<AsyncSmtpTransport<Tokio1Executor>>
}

impl AppState {
    pub fn new(jinja: Arc<AutoReloader>, mailer: Arc<AsyncSmtpTransport<Tokio1Executor>>) -> Self {
        AppState { jinja, mailer }
    }
}

pub fn create_reloader(template_path: String) -> AutoReloader {
    AutoReloader::new(move |notifier| {
        let mut env = Environment::new();
        env.set_loader(path_loader(&template_path));
        notifier.set_fast_reload(true);
        notifier.watch_path(&template_path, true);
        Ok(env)
    })
}

pub fn create_mailer() -> Result<AsyncSmtpTransport<Tokio1Executor>, AppError> {
    let username = std::env::var("EMAIL_USERNAME").expect("EMAIL_USERNAME not set");
    let password = std::env::var("EMAIL_PASSWORD").expect("EMAIL_PASSWORD not set");
    let server = std::env::var("EMAIL_SERVER").expect("EMAIL_SERVER not set");
    
    let creds = Credentials::new(username, password);
    
    Ok(AsyncSmtpTransport::<Tokio1Executor>::relay(server.as_str())?
        .credentials(creds)
        .build())
}