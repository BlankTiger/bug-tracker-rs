use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
enum ProjectStatus {
    Active,
    Canceled,
    Pending,
    Upcoming,
    Overdue,
    Priority,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Project {
    pub project_id: i32,
    pub project_name: String,
    pub desc_short: String,
    pub desc_long: Option<String>,
    pub status: String,
    pub assigned_to: Vec<Uuid>,
    pub created_by: Uuid,
    pub created_at: chrono::NaiveDateTime,
}
