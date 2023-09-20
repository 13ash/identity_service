use crate::common::app_state::AppState;
use crate::models::users::{NewUserModel, UserModel};
use crate::utils::hash::hash_password;
use actix_web::web::Json;
use actix_web::{web, HttpResponse, Responder};
use chrono::{Utc};
use std::error::Error;
use actix_web::http::StatusCode;
use diesel::RunQueryDsl;
use uuid::Uuid;

pub async fn user_register(
    data: web::Data<AppState>,
    body: Json<NewUserModel>,
) -> Result<impl Responder, Box<dyn Error>> {
    use crate::schema::users::dsl::users;

    let new_user_model = body.into_inner();
    let hash_params = hash_password(&*new_user_model.password, &*data.config.local_salt)?;

    let mut conn = data.db.get()?;

    let insert_user = UserModel {
        id: Uuid::new_v4(),
        email: new_user_model.email,
        username: new_user_model.username,
        hash: hash_params.hashed_data,
        random_salt: hash_params.random_salt,
        first_name: new_user_model.first_name,
        last_name: new_user_model.last_name,
        created_at: Utc::now().naive_utc(),
    };

    diesel::insert_into(users)
        .values(&insert_user)
        .execute(&mut conn)?;

    Ok(HttpResponse::Ok().body("User Created."))
}
