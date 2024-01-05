use serde::{Deserialize, Serialize};
use actix_web::web;

#[derive(Serialize, Deserialize, Debug)]
pub struct MemberRegisterForm {
    pub username: String,
    pub password: String,
    pub confirmation: String,
    pub name: String,
    pub info: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MemberResponse {
    pub member_id: i32,
    pub member_name: String,
    pub member_info: String,
}

#[derive(Serialize, Deserialize, Debug, sqlx::FromRow)]
pub struct User {
    pub username: String,
    pub member_id: Option<i32>,
    pub user_password: String,
}

// Form to enable members to sign in
#[derive(Serialize, Deserialize, Debug)]
pub struct MemberSigninForm {
pub username: String,
pub password: String,
}
