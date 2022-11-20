use crate::{error::AppError, models::auth::Claims};

pub async fn user_profile(claims: Claims) -> Result<axum::Json<serde_json::Value>, AppError> {
    Ok(axum::Json(serde_json::json!({"email": claims.email})))
}