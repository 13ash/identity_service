use chrono::NaiveDateTime;
use uuid::Uuid;

pub struct UserQueryModel {
    pub id: Uuid,
}

pub struct UserModel {
    pub id: Uuid,
    pub email: String,
    pub username: String,
    pub hash: String,
    pub first_name: String,
    pub last_name: String,
    pub created_at: NaiveDateTime,
}

pub struct UserAuthModel {
    pub email: String,
    pub password: String,
}
