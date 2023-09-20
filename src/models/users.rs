use serde::{Deserialize, Serialize};
use diesel::{Insertable, Queryable};
use uuid::Uuid;
use chrono::NaiveDateTime;

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
    pub first_name: String,
    pub last_name: String,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Deserialize)]
pub struct UserAuthModel {
    pub email: String,
    pub password: String,
}
