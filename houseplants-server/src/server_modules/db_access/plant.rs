use crate::server_modules::model::plant::*;
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

// get plant details for particular member and particular plant 
pub async fn get_plant_details_db(pool: &PgPool, member_id: i32, plant_id: i32
) -> Plant {
    // Prepare SQL statement
    let plant = sqlx::query_as!(
        Plant, 
        "SELECT * FROM plant WHERE member_id = $1 AND plant_id = $2",
        member_id, plant_id
        )
        .fetch_optional(pool)
        .await
        .unwrap()
        .unwrap();

    plant
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

// delete plant record 
pub async fn delete_plant_db(
    pool: &PgPool,
    member_id: i32, 
    plant_id: i32, 
) -> String { 
    let plant = sqlx::query!(
        "DELETE FROM plant WHERE member_id = $1 AND plant_id = $2",
        member_id, 
        plant_id,
    )
    .execute(pool)
    .await
    .unwrap();
    format!("Deleted {:#?} record", plant)
}

// update plant record 
pub async fn update_plant_details_db (
    pool: &PgPool,
    member_id: i32,
    plant_id: i32, 
    update_plant: UpdatePlant,
) -> Plant {

    // Retrieve current plant record 
    let current_plant = sqlx::query_as!(
        Plant, 
        "SELECT * FROM plant WHERE member_id = $1 AND plant_id = $2",
        member_id,
        plant_id
    )
    .fetch_one(pool)
    .await
    .unwrap();

    // Construct the parameters for update: 
    let name: String = if let Some(name) = update_plant.plant_name {
        name 
    } else {current_plant.plant_name};
    let care: String = if let Some(care) = update_plant.plant_care {
        care
    } else {
        current_plant.plant_care.unwrap_or_default()
    };
    let care_difficulty: String = if let Some(care_difficulty) = update_plant.plant_care_difficulty {
        care_difficulty
    } else {
        current_plant.plant_care_difficulty.unwrap_or_default()
    };
    let alternative_name: String = if let Some(alternative_name) = update_plant.plant_alternative_name {
        alternative_name
    } else {
        current_plant.plant_alternative_name.unwrap_or_default()
    };
    let extra_info: String = if let Some(extra_info) = update_plant.plant_extra_info {
        extra_info
    } else {
        current_plant.plant_extra_info.unwrap_or_default()
    };
    let price = if let Some(price) = update_plant.plant_price {
        price
    } else {
        current_plant.plant_price.unwrap_or_default()
    };
    let description: String = if let Some(description) = update_plant.plant_description {
        description
    } else {
        current_plant.plant_description.unwrap_or_default()
    };

    // Prepare SQL statement 
    let plant = sqlx::query_as!(
        Plant,
        "UPDATE plant SET plant_name = $1, 
        plant_description = $2, plant_care = $3, 
        plant_care_difficulty = $4, plant_alternative_name = $5, 
        plant_price = $6, plant_extra_info = $7 
        WHERE member_id = $8 AND plant_id = $9 
        RETURNING 
        member_id, plant_id,
        plant_name, plant_description, plant_alternative_name, plant_extra_info,
        plant_care, plant_care_difficulty, plant_price, posted_time", 
        name, description, care,
        care_difficulty, alternative_name, price, extra_info, member_id, plant_id   
    )
        .fetch_one(pool)
        .await
        .unwrap();
    plant
}

// get all plants records from DB   
pub async fn get_plants_db(
    pool: &PgPool,
) -> Vec<Plant> {
    sqlx::query_as!(
        Plant, 
        "SELECT * FROM plant"
        )
        .fetch_all(pool)
        .await
        .unwrap()
}
