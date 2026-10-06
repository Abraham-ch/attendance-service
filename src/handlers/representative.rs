use axum::{
    Json,
    extract::{Path, State},
};
use uuid::Uuid;
use validator::Validate;

use crate::schema::{
    app::{AppError, AppState},
    representative::{
        CreateRepresentativeRequest, Representative, RepresentativeWithRelation,
        UpdateRepresentative,
    },
};

#[axum::debug_handler]
pub async fn list_representatives(
    State(state): State<AppState>,
) -> Result<Json<Vec<Representative>>, AppError> {
    let full_representatives = Representative::find_all(&state.pool).await?;
    Ok(Json(full_representatives))
}

#[axum::debug_handler]
pub async fn create_representative(
    State(state): State<AppState>,
    Json(new_representative): Json<CreateRepresentativeRequest>,
) -> Result<Json<RepresentativeWithRelation>, AppError> {
    new_representative.validate()?;
    let create_representative = Representative::new_with_relation(
        &state.pool,
        new_representative.representative,
        new_representative.relation,
    )
    .await?;

    Ok(Json(create_representative))
}

#[axum::debug_handler]
pub async fn get_representative_by_id(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<Representative>, AppError> {
    let representative_by_id = Representative::get_by_id(&state.pool, id).await?;
    Ok(Json(representative_by_id))
}

#[axum::debug_handler]
pub async fn update_representative(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(representative_to_update): Json<UpdateRepresentative>,
) -> Result<Json<UpdateRepresentative>, AppError> {
    representative_to_update.validate()?;
    let updated_representative =
        Representative::update(&state.pool, id, representative_to_update).await?;

    Ok(Json(updated_representative))
}

#[axum::debug_handler]
pub async fn delete_representative(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<String, AppError> {
    Representative::delete(&state.pool, id).await?;
    Ok("Representative deleted".to_string())
}
