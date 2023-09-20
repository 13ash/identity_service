use chrono::NaiveDateTime;
use diesel::{Insertable, Queryable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Queryable)]
pub struct UserQueryModel {
    pub id: Uuid,
}
#[derive(Debug, Serialize, Deserialize, Insertable)]
#[diesel(table_name = crate::schema::users)]
pub struct UserModel {
    pub id: Uuid,
    pub email: String,
    pub username: String,
    pub hash: String,
    pub random_salt: String,
    pub first_name: String,
    pub last_name: String,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Deserialize)]
pub struct UserAuthModel {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct NewUserModel {
    pub email: String,
    pub username: String,
    pub password: String,
    pub first_name: String,
    pub last_name: String,
}
