use crate::db_access::get_user_db;
use crate::errors::CustomError;
use crate::helpers::get_server_port;
use crate::model::{NewPlant, NewPlantForm, NewPlantResponse, UpdatePlant, UpdatePlantResponse};
use crate::state::AppState;
use actix_web::{web, Error, HttpResponse, Result};
use serde_json::json;

pub async fn show_new_plant_form(tmpl: web::Data<tera::Tera>) -> Result<HttpResponse, Error> {
    let mut ctx = tera::Context::new();
    ctx.insert("error", "");
    ctx.insert("current_user_name", "");
    ctx.insert("current_plant_name", "");
    ctx.insert("current_plant_description", "");
    ctx.insert("current_plant_care", "");
    ctx.insert("current_plant_alternative_name", "");
    ctx.insert("current_plant_care_difficulty", "");
    ctx.insert("current_plant_price", "");
    ctx.insert("current_plant_extra_info", "");
    let s = tmpl
        .render("new_plant_form/new_plant.html", &ctx)
        .map_err(|_| CustomError::TeraError("Template error".to_string()))?;
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

// add new plant record , to be used from New Plant Form
pub async fn new_plant_addition(
    tmpl: web::Data<tera::Tera>,
    app_state: web::Data<AppState>,
    params: web::Form<NewPlantForm>,
) -> Result<HttpResponse, Error> {
    let resource_url = format!("http://{}/plants/", get_server_port());
    let ctx = tera::Context::new();
    let username = params.user_name.clone();
    let user = get_user_db(&app_state.db, username.to_string()).await;

    match user {
        // if the user with the 'user_name' was found in DB
        Ok(user) => {
            let new_plant = json!({
              "member_id": user.member_id,
              "plant_name": &params.plant_name,
              "plant_description": &params.plant_description,
              "plant_care": &params.plant_care,
              "plant_alternative_name": &params.plant_alternative_name,
              "plant_care_difficulty": &params.plant_care_difficulty,
              "plant_price": &params.plant_price,
              "plant_extra_info": &params.plant_extra_info,
            });
            let awc_client = awc::Client::default();
            let res = awc_client
                .post(resource_url)
                .send_json(&new_plant)
                .await
                .unwrap()
                .body()
                .await?;
            println!("Finished call from DB with new plant as result: {:?}", res);
            let plant_response: NewPlantResponse =
                serde_json::from_str(std::str::from_utf8(&res)?)?;
            Ok(HttpResponse::Ok().json(plant_response))
        }
        // if we can not find user with the 'user_name' in DB
        Err(_) => {
            let s = tmpl
                .render("new_plant_form/new_plant.html", &ctx)
                .map_err(|_| {
                    CustomError::TeraError("Template error, user was not found in DB".to_string())
                })?;
            Ok(HttpResponse::Ok().content_type("text/html").body(s))
        }
    }
}

// add new plant record if web::Path is given ,
// can be used with curl
pub async fn handle_insert_plant(
    _tmpl: web::Data<tera::Tera>,
    _app_state: web::Data<AppState>,
    path: web::Path<i32>,
    params: web::Json<NewPlant>,
) -> Result<HttpResponse, Error> {
    let resource_url = format!("http://{}/plants/", get_server_port());
    let member_id = path.into_inner();
    let new_plant = json!({
      "member_id": member_id,
      "plant_name": &params.plant_name,
      "plant_description": &params.plant_description,
      "plant_care": &params.plant_care,
      "plant_care_difficulty": &params.plant_care_difficulty,
      "plant_alternative_name": &params.plant_alternative_name,
      "plant_price": &params.plant_price,
      "plant_extra_info": &params.plant_extra_info
    });
    let awc_client = awc::Client::default();
    let res = awc_client
        .post(resource_url)
        .send_json(&new_plant)
        .await
        .unwrap()
        .body()
        .await?;
    println!("Finished call: {:?}", res);
    let plant_response: NewPlantResponse = serde_json::from_str(std::str::from_utf8(&res)?)?;
    Ok(HttpResponse::Ok().json(plant_response))
}

// delete a plant for a member_id
pub async fn handle_delete_plant(
    _tmpl: web::Data<tera::Tera>,
    _app_state: web::Data<AppState>,
    path: web::Path<(i32, i32)>,
) -> Result<HttpResponse, Error> {
    let (member_id, plant_id) = path.into_inner();
    let delete_url = format!(
        "http://{}/plants/{}/{}",
        get_server_port(),
        member_id,
        plant_id
    );

    let awc_client = awc::Client::default();
    let _res = awc_client.delete(delete_url).send().await.unwrap();
    Ok(HttpResponse::Ok().body("Plant record deleted "))
}

// update already existing plant record using the web::Path
pub async fn handle_update_plant(
    _tmpl: web::Data<tera::Tera>,
    _app_state: web::Data<AppState>,
    path: web::Path<(i32, i32)>,
    params: web::Json<UpdatePlant>,
) -> Result<HttpResponse, Error> {
    let (member_id, plant_id) = path.into_inner();
    let update_plant = json!({
      "plant_name": &params.plant_name,
      "plant_description": &params.plant_description,
      "plant_care": &params.plant_care,
      "plant_alternative_name": &params.plant_alternative_name,
      "plant_care_difficulty": &params.plant_care_difficulty,
      "plant_price": &params.plant_price,
      "plant_extra_info": &params.plant_extra_info,
    });
    let awc_client = awc::Client::default();
    let update_url = format!(
        "http://{}/plants/{}/{}",
        get_server_port(),
        member_id,
        plant_id
    );
    let res = awc_client
        .put(update_url)
        .send_json(&update_plant)
        .await
        .unwrap()
        .body()
        .await?;
    let plant: UpdatePlantResponse = serde_json::from_str(std::str::from_utf8(&res)?)?;
    Ok(HttpResponse::Ok().json(plant))
}

// show particular houseplant record for particular member
// return is in 'row' format, without a template rendering
pub async fn show_plant_for_member(
    _tmpl: web::Data<tera::Tera>,
    _app_state: web::Data<AppState>,
    path: web::Path<(i32, i32)>,
) -> Result<HttpResponse, Error> {
    let (member_id, plant_id) = path.into_inner();
    let awc_client = awc::Client::default();
    let member_plant_url = format!(
        "http://{}/plants/{}/{}",
        get_server_port(),
        member_id,
        plant_id
    );
    let res = awc_client
        .get(member_plant_url)
        .send()
        .await
        .unwrap()
        .body()
        .await?;
    let plant_response: NewPlantResponse = serde_json::from_str(std::str::from_utf8(&res)?)?;
    Ok(HttpResponse::Ok().json(plant_response))
}

// show particular houseplant record for particular member
// Tera template is used for handling the view
pub async fn show_plant_for_member_render_template(
    tmpl: web::Data<tera::Tera>,
    _app_state: web::Data<AppState>,
    path: web::Path<(i32, i32)>,
) -> Result<HttpResponse, Error> {
    let (member_id, plant_id) = path.into_inner();
    let awc_client = awc::Client::default();
    let member_plant_url = format!(
        "http://{}/plants/{}/{}",
        get_server_port(),
        member_id,
        plant_id
    );
    let res = awc_client
        .get(member_plant_url)
        .send()
        .await
        .unwrap()
        .body()
        .await?;
    let plant_response: NewPlantResponse = serde_json::from_str(std::str::from_utf8(&res)?)?;
    let mut ctx = tera::Context::new();
    ctx.insert("current_name", &plant_response.plant_name);
    ctx.insert(
        "current_alternative_name",
        &plant_response.plant_alternative_name,
    );
    ctx.insert("current_description", &plant_response.plant_description);
    ctx.insert("current_care", &plant_response.plant_care);
    ctx.insert(
        "current_care_difficulty",
        &plant_response.plant_care_difficulty,
    );
    ctx.insert("current_price", &plant_response.plant_price);
    ctx.insert("current_extra_info", &plant_response.plant_extra_info);
    let s = tmpl
        .render("plant_for_member/plant_for_member.html", &ctx)
        .map_err(|_| CustomError::TeraError("Template error".to_string()))?;
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

// view list of all plant records in House Plants
// Tera template is used for handling the view
pub async fn show_plants(tmpl: web::Data<tera::Tera>) -> Result<HttpResponse, Error> {
    let awc_client = awc::Client::default();
    let plants_url = format!("http://{}/plants/", get_server_port());
    let res = awc_client
        .get(plants_url)
        .send()
        .await
        .unwrap()
        .body()
        .await?;
    let plants_response: Vec<NewPlantResponse> = serde_json::from_str(std::str::from_utf8(&res)?)?;
    let mut ctx = tera::Context::new();
    ctx.insert("plants", &plants_response);
    let s = tmpl
        .render("plants/plants.html", &ctx)
        .map_err(|_| CustomError::TeraError("Template error".to_string()))?;
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}
