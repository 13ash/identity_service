use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Serialize)]
pub struct StatusModel {
    pub id: Uuid,
    pub status: String,
}
