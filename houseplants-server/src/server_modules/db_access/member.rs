use crate::server_modules::model::member::{Member, NewMember, UpdateMember};
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

pub async fn get_member_details_db(pool: &PgPool, member_id: i32) -> 
  Member {

    // Prepare SQL statement 
    let member = sqlx::query!(
      "SELECT member_id, member_name, member_info FROM member 
       WHERE member_id = $1 
       ORDER BY member_id DESC", member_id
    )
    .fetch_one(pool)
    .await
    .unwrap();
  Member {
    member_id: member.member_id,
    member_name: member.member_name,
    member_info: member.member_info
  }
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

pub async fn delete_member_db(pool: &PgPool, member_id: i32) -> 
  String {
    let member = sqlx::query(&format!(
      "DELETE FROM member WHERE member_id = {}", member_id
    ))
    .execute(pool)
    .await
    .unwrap();
    format!("Deleted {:#?} record", member)
}

pub async fn update_member_details_db (
  pool: &PgPool,
  member_id: i32,
  update_member: UpdateMember,
) -> Member {
  // Retrieve current member from DB
  let current_member = sqlx::query!(
    "SELECT member_id, member_name, member_info 
    FROM member 
    WHERE member_id = $1", member_id
  )
  .fetch_one(pool)
  .await
  .unwrap();

  let new_member = Member {
    member_id: current_member.member_id,
    member_name: match update_member.member_name {
      Some (name) => name, 
      None => current_member.member_name
    }, 
      member_info: match update_member.member_info {
        Some(info) => info, 
        None => current_member.member_info
      } 
    };

    // Prepare SQL update statement
    let _member_updated = sqlx::query!(
      "UPDATE member 
       SET member_name = $1, member_info = $2 
       WHERE member_id = $3
       RETURNING member_id, member_name, member_info",
       new_member.member_name, 
       new_member.member_info,  
       new_member.member_id)
       .fetch_one(pool)
       .await;
      Member {
        member_id: new_member.member_id, 
        member_name: new_member.member_name,
        member_info: new_member.member_info
      }
}
