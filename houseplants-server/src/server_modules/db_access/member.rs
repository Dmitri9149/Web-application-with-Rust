use crate::models::member::{Member};
use sqlx::postgres::PgPool;
use std::error::Error;

// get records of all members of the app 
pub async fn get_members_db(pool: &PgPool) -> Vec<Member> {
  // Prepare SQL statement
  let member_rows = 
    sqlx::query!("SELECT member_id, member_name, member_info 
      FROM member")
      .fetch_all(pool)
      .await
      .unwrap();

    // Extract result
    member_rows 
      .iter()
      .map (|member_row| Member {
        member_id: member_row.member_id,
        member_name: member_row.member_name.clone(),
        member_info: member_row.member_info.clone()

      })
      .collect()

}