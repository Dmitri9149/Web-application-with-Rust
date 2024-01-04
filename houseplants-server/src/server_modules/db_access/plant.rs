use crate::models::plant::*;
use sqlx::postgres::PgPool;
use std::error::Error;

// get all plants (records) created by a member  
pub async fn get_plants_for_member_db(
    pool: &PgPool,
    member_id: i32,
) -> Vec<Plant> {
    sqlx::query_as!(
        Plant, 
        "SELECT * FROM plant WHERE member_id = $1",
        member_id
        )
        .fetch_all(pool)
        .await
        .unwrap()
}

// post new plant record by a member
pub async fn post_new_plant_db(
    pool: &PgPool,
    new_plant: NewPlant,
) -> Plant {
    let plant_row= sqlx::query_as!(Plant,
        "INSERT INTO plant (member_id, plant_name, 
            plant_description, plant_alternative_name, plant_extra_info, plant_care, 
            plant_care_difficulty, plant_price) 
            values ($1,$2,$3,$4,$5,$6,$7,$8) 
            RETURNING member_id, plant_id, plant_name, plant_description, 
            plant_alternative_name, plant_extra_info, plant_care, 
            plant_care_difficulty, plant_price, posted_time", 
            new_plant.member_id, new_plant.plant_name, new_plant.plant_description,
            new_plant.plant_alternative_name, new_plant.plant_extra_info, new_plant.plant_care, 
            new_plant.plant_care_difficulty, new_plant.plant_price)
            .fetch_one(pool)
            .await
            .unwrap();
    plant_row
}

pub async fn delete_plant_db(
    pool: &PgPool,
    member_id: i32, 
    plant_id: i32, 
) -> String { 
    let plant_row = sqlx::query!(
        "DELETE FROM plant WHERE member_id = $1 AND plant_id = $2",
        member_id, 
        plant_id,
    )
    .execute(pool)
    .await
    .unwrap();
    format!("Deleted {:#?} record", plant_row)
}