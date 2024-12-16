use crate::error::AppError;
use crate::handlers::helpers::get_template_name;
use crate::templates::index::IndexTemplate;
use crate::AppState;
use actix_web::{get, web, HttpRequest, HttpResponse};

#[get("")]
pub async fn index(
    request: HttpRequest,
    state: web::Data<AppState>,
) -> Result<HttpResponse, AppError> {
    
    let template_name = get_template_name(&request, "index");
    println!("TEMPLATEEE {}", template_name);
    let env = state.jinja.acquire_env()?;
    let template = env.get_template(&template_name)?;
    let body = template.render(IndexTemplate {
        logged_in: false,
    })?;
    
    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}
