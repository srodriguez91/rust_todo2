use axum::Json;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub enum StatusEnum {
    ToDo,
    InProgress,
    Completed,
}

#[derive(Serialize)]
pub struct Task {
    id: Option<i32>,
    description: String,
    status: StatusEnum,
}

#[derive(Deserialize)]
pub struct CreateTaskRequest {
    pub description: String,
}

impl Task {
    pub async fn create_task(Json(req): Json<CreateTaskRequest>) -> Json<Task> {
        Json(Task {
            id: Some(1),
            description: req.description,
            status: StatusEnum::ToDo,
        })
    }
}
