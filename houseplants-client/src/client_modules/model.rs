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

// Struct to hold user-provided details to create a new plant
#[derive(Deserialize, Debug, Clone)]
pub struct NewPlant {
    pub plant_name: String,
    pub plant_description: String,
    pub plant_care: String,
    pub plant_alternative_name: String,
    pub plant_care_difficulty: Option<String>,
    pub plant_price: Option<i32>,
    pub plant_extra_info: Option<String>,
}

// Struct to hold the response from server after creation of a new plant
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct NewPlantResponse {
    pub plant_id: i32,
    pub member_id: i32,
    pub plant_name: String,
    pub plant_description: String,
    pub plant_care: String,
    pub plant_care_difficulty: Option<String>,
    pub plant_alternative_name: String,
    pub plant_price: Option<i32>,
    pub plant_extra_info: Option<String>,
    pub posted_time: String,
}

// From trait implementation to convert json response from server into a Rust struct
impl From<web::Json<NewPlantResponse>> for NewPlantResponse {
    fn from(new_plant: web::Json<NewPlantResponse>) -> Self {
        NewPlantResponse {
            member_id: new_plant.member_id,
            plant_id: new_plant.plant_id,
            plant_name: new_plant.plant_name.clone(),
            plant_description: new_plant.plant_description.clone(),
            plant_care: new_plant.plant_care.clone(),
            plant_care_difficulty: new_plant.plant_care_difficulty.clone(),
            plant_alternative_name: new_plant.plant_alternative_name.clone(),
            plant_price: new_plant.plant_price,
            plant_extra_info: new_plant.plant_extra_info.clone(),
            posted_time: new_plant.posted_time.clone(),
        }
    }
}

// Struct to hold the user-provided plant details for update
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct UpdatePlant {
    pub plant_name: Option<String>,
    pub plant_description: Option<String>,
    pub plant_care: Option<String>,
    pub plant_alternative_name: Option<String>,
    pub plant_care_difficulty: Option<String>,
    pub plant_price: Option<i32>,
    pub plant_extra_info: Option<String>,
}

// Struct to hold the response from server after plant update
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct UpdatePlantResponse {
    pub plant_id: i32,
    pub member_id: i32,
    pub plant_name: String,
    pub plant_description: String,
    pub plant_care: String,
    pub plant_care_difficulty: String,
    pub plant_alternative_name: String,
    pub plant_price: i32,
    pub plant_extra_info: String,
    pub posted_time: String,
}

// From trait implementation to convert the json response received from the server to
// Rust struct
impl From<web::Json<UpdatePlantResponse>> for UpdatePlantResponse {
    fn from(new_plant: web::Json<UpdatePlantResponse>) -> Self {
        UpdatePlantResponse {
            member_id: new_plant.member_id,
            plant_id: new_plant.plant_id,
            plant_name: new_plant.plant_name.clone(),
            plant_description: new_plant.plant_description.clone(),
            plant_care: new_plant.plant_care.clone(),
            plant_care_difficulty: new_plant.plant_care_difficulty.clone(),
            plant_alternative_name: new_plant.plant_alternative_name.clone(),
            plant_price: new_plant.plant_price,
            plant_extra_info: new_plant.plant_extra_info.clone(),
            posted_time: new_plant.posted_time.clone(),
        }
    }
}

// Struct to handle plants records from form
#[derive(Serialize, Deserialize, Debug)]
pub struct NewPlantForm {
    pub user_name: String,
    pub plant_name: String,
    pub plant_description: String,
    pub plant_care: String,
    pub plant_alternative_name: String,
    pub plant_care_difficulty: Option<String>,
    pub plant_price: Option<i32>,
    pub plant_extra_info: Option<String>,
}

// Struct to hold user-provided details to create a new interesting fact record
#[derive(Deserialize, Debug, Clone)]
pub struct NewFact {
    pub content: String,
    pub ref_to_origin: String,
}

// Struct to hold the response from server after creation of a new interesting fact 
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct NewFactResponse {
    pub fact_id: i32,
    pub member_id: i32,
    pub content: String,
    pub ref_to_origin: String,
}

// From trait implementation to convert json response from server into a Rust struct
impl From<web::Json<NewFactResponse>> for NewFactResponse {
    fn from(new_fact: web::Json<NewFactResponse>) -> Self {
        NewFactResponse {
            member_id: new_fact.member_id,
            fact_id: new_fact.fact_id,
            content: new_fact.content.clone(),
            ref_to_origin: new_fact.ref_to_origin.clone(),
        }
    }
}

