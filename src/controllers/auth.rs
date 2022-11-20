use axum::{Json, Extension};
use jsonwebtoken::{encode, Header};
use serde_json::{json, Value};
use sqlx::PgPool;

use crate::{
    error::AppError,
    models::{self, auth::Claims},
    utils::get_timestamp_8_hours_from_now,
    KEYS
};

pub async fn login(
    Json(credentials): Json<models::auth::User>,
    Extension(pool): Extension<PgPool>,
) -> Result<Json<Value>, AppError> {
    if credentials.email.is_empty() || credentials.password.is_empty() {
        return Err(AppError::MissingCredential);
    }
    
    let user = sqlx::query_as::<_, models::auth::User>(
        "SELECT email, password FROM users WHERE email = $1",
    ).bind(&credentials.email)
    .fetch_optional(&pool)
    .await
    .map_err(|err| {
        dbg!(err);
        AppError::InternalServerError
    })?;

    if let Some(user) = user {
        if user.password != credentials.password {
            Err(AppError::WrongCredential)
        } else {
            let claims = Claims {
                email: credentials.email.to_owned(),
                exp: get_timestamp_8_hours_from_now(),
            };
            let token = encode(&Header::default(), &claims, &KEYS.encoding)
            .map_err(|err| AppError::TokenCreation)?;

            Ok(Json(json!({ "access_token": token, "type": "Bearer" })))
        }
    } else {
        Err(AppError::UserDoeNotExist)
    }
}

pub async fn register(
    Json(credentials): Json<models::auth::User>,
    Extension(pool): Extension<PgPool>,
) -> Result<Json<Value>, AppError> {
    if credentials.email.is_empty() || credentials.password.is_empty() {
        return Err(AppError::MissingCredential);
    }

    let user = sqlx::query_as::<_, models::auth::User>(
        "SELECT email, password FROM users WHERE email = $1",
    )
        .bind(&credentials.email)
        .fetch_optional(&pool)
        .await
        .map_err(|err| {dbg!(err); AppError::InternalServerError})?;

    let result = sqlx::query("INSERT INTO users (email, password) VALUES ($1, $2)")
        .bind(&credentials.email)
        .bind(credentials.password)
        .execute(&pool)
        .await
        .map_err(|_| AppError::InternalServerError)?;
    
        if result.rows_affected() < 1 {
            Err(AppError::InternalServerError)
        } else { 
            Ok(Json(json!({ "msg": "registered successfully" })))
        }
}