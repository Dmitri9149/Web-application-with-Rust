use crate::errors::CustomError;
use crate::model::*;
use sqlx::postgres::PgPool;

//Query user record from DB 
pub async fn get_user_db(pool: &PgPool, username: String) -> 
  Result<User, CustomError> {
  // Prepare SQL statement
  let result = sqlx::query_as!(
    User,
    "SELECT * FROM web_user WHERE username = $1",
    username  
  )
  .fetch_optional(pool)
  .await?;

  if let Some(user) = result {
    Ok(user)
  } else {
    Err(CustomError::NotFound("Username not found".into()))
  }
}

pub async fn post_new_user_db(pool: &PgPool, new_user: User) -> 
  Result<User, CustomError> {
    let user = sqlx::query_as!(
      User, 
      "INSERT INTO web_user 
      (username, member_id, user_password) VALUES ($1, $2, $3)
      RETURNING username, member_id, user_password",
      new_user.username, new_user.member_id, new_user.user_password
    )
    .fetch_one(pool)
    .await?;

    Ok(user)
}