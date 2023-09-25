use crate::common::app_state::AppState;
use crate::models::users::{NewUserModel, UserAuthModel, UserModel};
use crate::stubs::presence::UserIdentifier;
use actix_web::{web, web::Json, HttpResponse, Responder};
use chrono::Utc;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use std::error::Error;
use tonic::Response;
use crate::stubs::auth::{ActionResult, AuthRequest, AuthResponse};
use uuid::Uuid;

pub async fn user_register(
    data: web::Data<AppState>,
    body: Json<NewUserModel>,
) -> Result<impl Responder, Box<dyn Error>> {
    use crate::schema::users::dsl::users;

    let new_user_model = body.into_inner();

    let mut conn = data.db.get()?;

    let insert_user = UserModel {
        id: Uuid::new_v4(),
        email: new_user_model.email,
        username: new_user_model.username,
        first_name: new_user_model.first_name,
        last_name: new_user_model.last_name,
        created_at: Utc::now().naive_utc(),
    };

    diesel::insert_into(users)
        .values(&insert_user)
        .execute(&mut conn)?;

    data.presence_client
        .lock()
        .await
        .add(UserIdentifier {
            uuid: insert_user.id.to_string(),
        })
        .await?;

    data.auth_client
        .lock()
        .await
        .add(AuthRequest {
            uuid: insert_user.id.to_string(),
            password: new_user_model.password,
        })
        .await?;

    Ok(HttpResponse::Ok().body("User Created."))
}

pub async fn user_login(
    data: web::Data<AppState>,
    body: Json<UserAuthModel>,
) -> Result<impl Responder, Box<dyn Error>> {
    use crate::schema::users::dsl::*;

    let auth_model = body.into_inner();
    let mut conn = data.db.get()?;

    let maybe_user_in_db: Result<UserModel, diesel::result::Error> =
        users.filter(email.eq(&auth_model.email)).first(&mut conn);

    match maybe_user_in_db {
        Ok(user_in_db) => {
            let authenticate_response: Response<AuthResponse> = data
                .auth_client
                .lock()
                .await
                .authenticate(AuthRequest {
                    uuid: user_in_db.id.to_string(),
                    password: auth_model.password,
                })
                .await?;


            if authenticate_response.into_inner().result == ActionResult::Authenticated as i32 {
                data.presence_client
                    .lock()
                    .await
                    .login(UserIdentifier {
                        uuid: user_in_db.id.to_string(),
                    })
                    .await?;
                Ok(HttpResponse::Ok().json("Authenticated"))
            } else {
                Ok(HttpResponse::Unauthorized().json("Wrong Password"))
            }
        }
        Err(_) => Ok(HttpResponse::NotFound().json("User not found")),
    }
}

pub async fn user_logout(
    data: web::Data<AppState>,
    body: Json<UserAuthModel>,
) -> Result<impl Responder, Box<dyn Error>> {
    use crate::schema::users::dsl::users;
    use crate::schema::users::email;

    let auth_model = body.into_inner();
    let mut conn = data.db.get()?;
    let user_in_db: UserModel = users.filter(email.eq(&auth_model.email)).first(&mut conn)?;

    data.presence_client
        .lock()
        .await
        .logout(UserIdentifier {
            uuid: user_in_db.id.to_string(),
        })
        .await?;

    Ok(HttpResponse::Ok().body("Logged out"))
}
