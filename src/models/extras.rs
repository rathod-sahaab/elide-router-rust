use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct AppError {
    pub error: String,
}
