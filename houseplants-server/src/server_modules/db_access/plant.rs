use crate::models::plant::*;
use sqlx::postgres::PgPool;
use std::error::Error;

// get all plants (records) created by a member  
pub async fn get_plants_for_member_db(
    pool: &PgPool,
    member_id: i32,
) -> Vec<Plant> {
    // Prepare SQL statement
    let plant_rows: Vec<Plant> = sqlx::query_as!(
        Plant, 
        "SELECT * FROM plant where member_id = $1",
        member_id
    )
    .fetch_all(pool)
    .await
    .unwrap();
    // Extract result
    plant_rows
}