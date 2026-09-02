use axum::{Json, extract::{Path, State}, http::StatusCode};
use uuid::Uuid;
use validator::Validate;

use crate::{repository::representative::{create_one_with_relation, delete_one, find_all, get_by_id, update_one}, schema::{app::AppState, representative::{CreateRepresentativeRequest, Representative, RepresentativeWithRelation, UpdateRepresentative}}};

#[axum::debug_handler]
pub async fn list_representatives(State(state): State<AppState>) -> Result<(StatusCode, Json<Vec<Representative>>), (StatusCode, String)>{ 

    let full_representatives = find_all(&state.pool).await;

    match full_representatives {
        Ok(result) => Ok((StatusCode::OK, Json(result))),
        Err(_) => Err((StatusCode::NOT_FOUND, "Representative not found".to_string()))
    }
}

#[axum::debug_handler]
pub async fn create_representative(State(state): State<AppState>, Json(new_representative): Json<CreateRepresentativeRequest>) -> Result<(StatusCode, Json<RepresentativeWithRelation>), (StatusCode, String)> {

    new_representative.validate().map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;
    let create_representative: Result<RepresentativeWithRelation, sqlx::Error> = create_one_with_relation(&state.pool, new_representative.representative, new_representative.relation).await;

    match create_representative {
        Ok(result) => Ok((StatusCode::CREATED, Json(result))),
        Err(sqlx::Error::Database(db_err)) if db_err.is_unique_violation() =>
        Err((StatusCode::CONFLICT, "dni already exists".to_string())),
        Err(_) => Err((StatusCode::INTERNAL_SERVER_ERROR, "Failed to create representative.".to_string()))
    }
}

#[axum::debug_handler]
pub async fn get_representative_by_id(State(state): State<AppState>, Path(id): Path<Uuid>) -> Result<(StatusCode, Json<Representative>), (StatusCode, String)> {

    let representative_by_id = get_by_id(&state.pool, id).await;

    match representative_by_id {
        Ok(result) => Ok((StatusCode::OK, Json(result))),
        Err(_) => Err((StatusCode::NOT_FOUND, "Representative not found".to_string()))
    }
}

#[axum::debug_handler]
pub async fn update_representative(State(state): State<AppState>, Path(id): Path<Uuid>, Json(representative_to_update): Json<UpdateRepresentative>) -> Result<(StatusCode, Json<UpdateRepresentative>), (StatusCode, String)> {

    representative_to_update.validate().map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;
    let updated_representative = update_one(&state.pool, id, representative_to_update).await;
    
    match updated_representative {
        Ok(result) => Ok((StatusCode::OK, Json(result))),
        Err(sqlx::Error::RowNotFound) => Err((StatusCode::NOT_FOUND, "Representative not found".to_string())),
        Err(_) => Err((StatusCode::INTERNAL_SERVER_ERROR, "Failed to update representative".to_string()))
    }
}

#[axum::debug_handler]
pub async fn delete_representative(State(state): State<AppState>, Path(id): Path<Uuid>) -> Result<(StatusCode, String), (StatusCode, String)> {

    let deleted_representative = delete_one(&state.pool, id).await;

    match deleted_representative {
        Ok(result) if result.rows_affected() == 0 => Err((StatusCode::NOT_FOUND, "Representative not found".to_string())),
        Ok(_) => Ok((StatusCode::OK, "Representative deleted".to_string())),
        Err(_) => Err((StatusCode::INTERNAL_SERVER_ERROR, "Failed to delete representative.".to_string()))
    }
}
