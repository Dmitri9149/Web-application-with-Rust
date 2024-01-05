use crate::errors::CustomError;
use crate::model::*;
use sqlx::postgres::PgPool;

//Query user record from DB 
pub async fn get_user_record_db(pool: &PgPool, username: String) -> 
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