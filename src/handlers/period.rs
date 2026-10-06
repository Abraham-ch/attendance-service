use axum::{
    Json,
    extract::{Path, State},
};
use uuid::Uuid;

use crate::schema::{
    app::{AppError, AppState},
    period::{NewPeriod, Period},
};

#[axum::debug_handler]
pub async fn create_period(
    State(state): State<AppState>,
    Json(period): Json<NewPeriod>,
) -> Result<Json<Period>, AppError> {
    //TODO: add fn to validate the date of period.validate()?;
    let new_period = Period::new(&state.pool, period).await?;
    Ok(Json(new_period))
}

pub async fn get_periods(State(state): State<AppState>) -> Result<Json<Vec<Period>>, AppError> {
    let periods = Period::get_all(&state.pool).await?;
    Ok(Json(periods))
}

pub async fn delete_period(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<String, AppError> {
    Period::delete(&state.pool, id).await?;
    Ok(format!("Period with id {:?} deleted sucsesfully.", id))
}
