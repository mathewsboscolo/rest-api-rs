use std::time::{Duration, SystemTime, UNIX_EPOCH};

use axum::{
    async_trait,
    extract::{FromRequest, RequestParts},
    headers::{authorization::{Bearer, self}, Authorization}, TypedHeader,
};

use jsonwebtoken::{decode, Validation};
use crate::{error::AppError, models::auth::Claims, KEYS};

pub fn get_timestamp_8_hours_from_now() -> u64 {
    let now = SystemTime::now();
    let since_the_epoch = now.duration_since(UNIX_EPOCH).expect("Time is wrong");

    let eight_from_now = since_the_epoch + Duration::from_secs(28800);

    eight_from_now.as_secs()
}

#[async_trait]
impl<B> FromRequest<B> for Claims 
where 
    B: Send, 
{
    type Rejection = AppError;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let bearer = TypedHeader::<Authorization<Bearer>>::from_request(req)
                .await
                .map_err(|_| AppError::InvalidToken)?;
                
        let data = decode::<Claims>(
            bearer.token(), &KEYS.decoding, &Validation::default()
        ).map_err(|_| AppError::InvalidToken)?;
            
        Ok(data.claims)
    }
}