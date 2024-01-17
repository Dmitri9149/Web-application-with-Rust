use crate::db_access::interesting_fact::*;
use crate::state::AppState;
use actix_web::{web, HttpResponse};

pub async fn get_interesting_facts(app_state: web::Data<AppState>) -> HttpResponse {
    let facts = get_interesting_facts_db(&app_state.db).await;
    HttpResponse::Ok().json(facts)
}
