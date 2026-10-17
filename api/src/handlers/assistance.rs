use axum::{
    Extension, Json,
    extract::{
        Path, Query, State, WebSocketUpgrade,
        ws::{Message, WebSocket},
    },
    response::Response,
};
use futures_util::{
    SinkExt,
    stream::{SplitSink, SplitStream, StreamExt},
};
use tokio::sync::broadcast::{Receiver, error::RecvError};
use uuid::Uuid;

use chrono::Utc;

use crate::{
    schema::{
        app::{AppError, AppState},
        assistance::{Assistance, CheckIn, DailyCode, Location, NewAssistance},
        period::Period,
        user::{Claims, User},
    },
    utils::{check_in_radius_m, daily_code, daily_code_origin, distance_m, valid_location},
};

pub async fn get_period_assistances(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<Vec<Assistance>>, AppError> {
    let assistances = Assistance::get_by_period(&state.pool, id).await?;
    Ok(Json(assistances))
}

/// GET /assistance/daily-code?lat=&lng=, admins show it as a QR for students to check in with.
/// The location is where the QR is shown, students must be near it to use it.
pub async fn get_daily_code(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Query(location): Query<Location>,
) -> Result<Json<DailyCode>, AppError> {
    let spot = (location.lat, location.lng);
    if !valid_location(spot) {
        return Err(AppError::BadRequest("Invalid location"));
    }
    let user_id: Uuid = claims.id.parse().map_err(|_| AppError::Unauthenticated)?;
    let date = Utc::now().date_naive(); // Assistance::new stores UTC dates too
    let code = daily_code(&state.secret, date, spot).map_err(|e| AppError::Internal(e.into()))?;
    DailyCode::record_location(&state.pool, user_id, date, spot).await?;
    Ok(Json(DailyCode { code, date }))
}

/// The student behind the logged-in account, None for staff accounts
async fn my_student_id(state: &AppState, claims: &Claims) -> Result<Option<Uuid>, AppError> {
    let user_id: Uuid = claims.id.parse().map_err(|_| AppError::Unauthenticated)?;
    Ok(User::get_by_id(&state.pool, user_id).await?.student_id)
}

/// POST /assistance/check-in, a student marks themselves present by scanning the daily QR.
/// The student comes from the token, never from the body, so nobody can check in someone else.
pub async fn check_in(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(body): Json<CheckIn>,
) -> Result<Json<Assistance>, AppError> {
    let today = Utc::now().date_naive();
    let Some(origin) = daily_code_origin(&state.secret, &body.code, today) else {
        return Err(AppError::BadRequest("That is not today's attendance QR"));
    };
    let here = (body.lat, body.lng);
    // written as !(a <= b) so a NaN distance is rejected too
    if !valid_location(here) || !(distance_m(origin, here) <= check_in_radius_m()) {
        return Err(AppError::BadRequest(
            "You have to be in class, near the QR, to check in",
        ));
    }
    let Some(student_id) = my_student_id(&state, &claims).await? else {
        return Err(AppError::BadRequest(
            "This account is not linked to a student",
        ));
    };
    let period = Period::current(&state.pool).await.map_err(|e| match e {
        sqlx::Error::RowNotFound => AppError::BadRequest("There is no period running today"),
        e => e.into(),
    })?;
    // UNIQUE (student_id, date) from migration 0012 turns a second scan into a Conflict
    let assistance = Assistance::new(
        &state.pool,
        NewAssistance {
            student_id,
            period_id: period.id,
        },
    )
    .await
    .map_err(|e| match AppError::from(e) {
        AppError::Conflict => AppError::BadRequest("You are already on today's roll"),
        e => e,
    })?;
    if let Ok(json) = serde_json::to_string(&assistance) {
        let _ = state.tx.send(json); // admins watching the roll see it live
    }
    Ok(Json(assistance))
}

/// GET /assistance/me, the logged-in student's own records
pub async fn get_my_assistances(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<Vec<Assistance>>, AppError> {
    let assistances = match my_student_id(&state, &claims).await? {
        Some(student_id) => Assistance::get_by_student(&state.pool, student_id).await?,
        None => vec![],
    };
    Ok(Json(assistances))
}

#[axum::debug_handler]
pub async fn ws_handler(State(state): State<AppState>, ws: WebSocketUpgrade) -> Response {
    ws.on_upgrade(|socket| handle_socket(state, socket))
}

pub async fn handle_socket(state: AppState, socket: WebSocket) {
    let (sender, receiver) = socket.split();

    let rx = state.tx.subscribe();

    let mut write_task = tokio::spawn(write(sender, rx));
    let mut read_task = tokio::spawn(read(receiver, state));

    tokio::select! {
        _ = &mut write_task => read_task.abort(),
        _ = &mut read_task => write_task.abort(),
    }
}

async fn write(mut sender: SplitSink<WebSocket, Message>, mut rx: Receiver<String>) {
    loop {
        match rx.recv().await {
            Ok(msg) => {
                // If sending fails, the client is gone: stop instead of panicking
                if sender.send(Message::Text(msg.into())).await.is_err() {
                    break;
                }
            }
            Err(RecvError::Lagged(_)) => continue,
            Err(RecvError::Closed) => break,
        }
    }
}

async fn read(mut receiver: SplitStream<WebSocket>, state: AppState) {
    while let Some(Ok(msg)) = receiver.next().await {
        let Message::Text(content) = msg else {
            continue;
        };

        let scan: NewAssistance = match serde_json::from_str(content.as_str()) {
            Ok(scan) => scan,
            Err(e) => {
                eprintln!("invalid message: {e}");
                continue;
            }
        };

        match Assistance::new(&state.pool, scan).await {
            Ok(assistance) => {
                if let Ok(json) = serde_json::to_string(&assistance) {
                    let _ = state.tx.send(json);
                }
            }
            Err(e) => eprintln!("db error: {e}"),
        }
    }
}
