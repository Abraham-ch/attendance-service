use axum::{
    Json,
    extract::{
        Path, State, WebSocketUpgrade,
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

use crate::schema::{
    app::{AppError, AppState},
    assistance::{Assistance, NewAssistance},
};

pub async fn get_period_assistances(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<Vec<Assistance>>, AppError> {
    let assistances = Assistance::get_by_period(&state.pool, id).await?;
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
