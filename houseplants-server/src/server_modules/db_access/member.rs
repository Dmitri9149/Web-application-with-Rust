use crate::models::member::{Member, NewMember};
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

pub async fn post_new_member_db(pool: &PgPool, new_member: NewMember) -> 
  Member {
    let member_row = sqlx::query!(
      "INSERT INTO member (member_name, member_info) 
       VALUES ($1,$2) 
       RETURNING member_id, member_name, member_info", 
       new_member.member_name, new_member.member_info)
       .fetch_one(pool)
      .await
      .unwrap();
    Member {
      member_id: member_row.member_id,
      member_name: member_row.member_name,
      member_info: member_row.member_info,
      }
}