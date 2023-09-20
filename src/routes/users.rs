use crate::common::app_state::AppState;
use crate::models::users::{NewUserModel, UserAuthModel, UserModel};
use crate::utils::hash::{hash_password, verify_hashed_data, HashParams};
use actix_web::{http::StatusCode, web, web::Json, HttpResponse, Responder};
use chrono::Utc;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use std::error::Error;
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

pub async fn user_login(
    data: web::Data<AppState>,
    body: Json<UserAuthModel>,
) -> Result<impl Responder, Box<dyn Error>> {
    use crate::schema::users::dsl::*;
    use crate::schema::users::email;

    let auth_model = body.into_inner();
    let mut conn = data.db.get()?;

    let maybe_user_in_db: Result<UserModel, diesel::result::Error> =
        users.filter(email.eq(&auth_model.email)).first(&mut conn);

    match maybe_user_in_db {
        Ok(user_in_db) => {
            let authenticated = verify_hashed_data(
                HashParams {
                    hashed_data: user_in_db.hash,
                    random_salt: user_in_db.random_salt,
                    local_salt: data.config.local_salt.clone(),
                },
                &auth_model.password,
            )?;

            if authenticated {
                Ok(HttpResponse::Ok().json("Authenticated"))
            } else {
                Ok(HttpResponse::Unauthorized().json("Wrong Password"))
            }
        }
        Err(_) => Ok(HttpResponse::NotFound().json("User not found")),
    }
}
