use crate::errors::CustomError;
use crate::state::AppState;
use actix_web::{web, Error, HttpResponse};
use std::time::{SystemTime, UNIX_EPOCH};

// general route handler
pub async fn general_page_handler(
    app_state: web::Data<AppState>,
    tmpl: web::Data<tera::Tera>,
) -> Result<HttpResponse, Error> {
    let web_client_is_running = &app_state.web_client_is_running_message;
    let time_now = SystemTime::now();
    let since_the_epoch = time_now
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    let response = format!(
        "{};  Current time is: {:?}; Duration since UNIX EPOCH is {:?}",
        web_client_is_running, time_now, since_the_epoch
    );
    let mut ctx = tera::Context::new();
    ctx.insert("message", &response);
    let s = tmpl
        .render("general/general.html", &ctx)
        .map_err(|_| CustomError::TeraError("Template error".to_string()))?;
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}
