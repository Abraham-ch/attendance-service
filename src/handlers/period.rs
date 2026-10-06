use axum::{Json, extract::State, http::StatusCode};

use crate::schema::{
    app::AppState,
    period::{DeletePeriod, NewPeriod, Period},
};

#[axum::debug_handler]
pub async fn create_period(
    State(state): State<AppState>,
    Json(period): Json<NewPeriod>,
) -> Result<(StatusCode, Json<Period>), (StatusCode, String)> {
    //TODO: add fn to validate the date of period.validate()?;
    let new_period = Period::new(&state.pool, period).await;

    let result = match new_period {
        Ok(period) => Ok((StatusCode::CREATED, Json(period))),
        Err(err) => Err((StatusCode::INTERNAL_SERVER_ERROR, err.to_string())), //TODO: add more detailed error handling
    };

    result
}

pub async fn get_periods(
    State(state): State<AppState>,
) -> Result<(StatusCode, Json<Vec<Period>>), (StatusCode, String)> {
    let periods = Period::get_all(&state.pool).await;

    let result = match periods {
        Ok(periods) => Ok((StatusCode::OK, Json(periods))),
        Err(err) => Err((StatusCode::INTERNAL_SERVER_ERROR, err.to_string())), //TODO: add more detailed error handling
    };

    result
}

pub async fn delete_period(
    State(state): State<AppState>,
    Json(period): Json<DeletePeriod>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    let deleted_period = Period::delete(&state.pool, period.id).await;

    let result = match deleted_period {
        Ok(_) => Ok((
            StatusCode::OK,
            format!("Period with id {:?} deleted sucsesfully.", period.id),
        )),
        Err(err) => Err((StatusCode::INTERNAL_SERVER_ERROR, err.to_string())), //TODO: add more detailed error handling
    };

    result
}
