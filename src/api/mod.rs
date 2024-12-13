use serde::Serialize;

pub mod v1;
pub mod v2;

/// A generic API response structure for consistent JSON responses.
#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: T,
    pub message: Option<String>,
}
