use aide::OperationOutput;
use axum::{
    Json,
    extract::FromRef,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use schemars::JsonSchema;
use serde::Serialize;
use serde_json::json;
use sqlx::PgPool;

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
    pub secret: String,
}

impl FromRef<AppState> for PgPool {
    fn from_ref(state: &AppState) -> PgPool {
        state.pool.clone()
    }
}

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("Recurso no encontrado")]
    NotFound,
    #[error("Datos inválidos")]
    Validation(#[from] validator::ValidationErrors),
    #[error("No autenticado")]
    Unauthenticated,
    #[error("Sin permisos")]
    Forbidden,
    #[error("El registro ya existe")]
    Conflict,
    #[error("Error interno")]
    Internal(#[source] anyhow::Error),
}

impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        match &err {
            sqlx::Error::RowNotFound => AppError::NotFound,
            sqlx::Error::Database(e) if e.is_unique_violation() => AppError::Conflict,
            _ => AppError::Internal(err.into()),
        }
    }
}

impl From<jsonwebtoken::errors::Error> for AppError {
    fn from(_: jsonwebtoken::errors::Error) -> Self {
        AppError::Unauthenticated
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status = match &self {
            AppError::NotFound => StatusCode::NOT_FOUND,
            AppError::Validation(_) => StatusCode::UNPROCESSABLE_ENTITY,
            AppError::Unauthenticated => StatusCode::UNAUTHORIZED,
            AppError::Forbidden => StatusCode::FORBIDDEN,
            AppError::Conflict => StatusCode::CONFLICT,
            AppError::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };

        let body = match &self {
            AppError::Validation(errors) => {
                json!({ "message": self.to_string(), "fields": errors })
            }
            AppError::Internal(e) => {
                tracing::error!("{e:?}"); // aquí ves todo, como con format!
                let msg = if cfg!(debug_assertions) {
                    e.to_string()
                } else {
                    "Error interno".into()
                };
                json!({ "message": msg })
            }
            _ => json!({ "message": self.to_string() }),
        };

        (status, Json(body)).into_response()
    }
}

#[derive(Serialize, JsonSchema)]
pub struct ErrorBody {
    pub message: String,
    /// Only present on validation errors
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<serde_json::Value>,
}

impl OperationOutput for AppError {
    type Inner = ();

    fn inferred_responses(
        ctx: &mut aide::generate::GenContext,
        operation: &mut aide::openapi::Operation,
    ) -> Vec<(Option<u16>, aide::openapi::Response)> {
        let base = Json::<ErrorBody>::operation_response(ctx, operation).unwrap_or_default();

        let with = |description: &str| aide::openapi::Response {
            description: description.to_string(),
            ..base.clone()
        };

        vec![
            (Some(401), with("No autenticado")),
            (Some(403), with("Sin permisos")),
            (Some(404), with("Recurso no encontrado")),
            (Some(409), with("El registro ya existe")),
            (Some(422), with("Datos inválidos")),
            (Some(500), with("Error interno")),
        ]
    }
}
