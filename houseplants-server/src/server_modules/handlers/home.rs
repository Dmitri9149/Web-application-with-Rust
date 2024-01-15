use actix_web::{web, HttpResponse};

// handlers to 'home' scope 

pub async fn index() -> HttpResponse {
    HttpResponse::Ok().json("Hello World!")
}

pub async fn hello(path: web::Path<String>) -> HttpResponse {
    let name = path.into_inner();
    let response = format!("Hello {}!", &name);
    HttpResponse::Ok().json(&response)

}

