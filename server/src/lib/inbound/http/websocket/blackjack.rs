/*
   Module `create_author` specifies an HTTP handler for creating a new [Author], and the
   associated data structures.
*/

use axum::extract::{State, WebSocketUpgrade};
use axum::response::{IntoResponse, Response};
use serde::{Deserialize, Serialize};

use crate::domain::blackjack::models::game::{
    Card, GameOutcome, HitError, HitRequest, StandError, StandRequest,
};
use crate::domain::blackjack::ports::BlackjackService;
use crate::inbound::http::AppState;
use axum::extract::ws::{Message, WebSocket};

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub enum ApiError {
    InternalServerError(String),
    LogicError(String),
}

impl From<HitError> for ApiError {
    fn from(source: HitError) -> Self {
        match source {
            HitError::AlreadyStood => Self::LogicError("player already stood".to_string()),
            HitError::AlreadyBusted => Self::LogicError("player already busted".to_string()),
            HitError::Unknown(cause) => {
                Self::InternalServerError("Internal server error".to_string())
            }
        }
    }
}

impl From<StandError> for ApiError {
    fn from(source: StandError) -> Self {
        tracing::debug!("Error: {:?}", source);
        match source {
            StandError::AlreadyStood => Self::LogicError("player already stood".to_string()),
            StandError::AlreadyBusted => Self::LogicError("player already busted".to_string()),
            StandError::Unknown(cause) => {
                tracing::error!("{:?}\n{}", cause, cause.backtrace());
                Self::InternalServerError("Internal server error".to_string())
            }
        }
    }
}

/// 2. Into ApiError for errors.
/// 1. IntoResponse for ApiError
impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        use ApiError::*;
        match self {
            InternalServerError(e) => {
                tracing::error!("{}", e);
                r#"{"type":"error","code":"internal server error"}"#.into_response()
            }
            LogicError(message) => {
                format!("{{\"type\":\"error\",\"code\":\"{}\"}}", message).into_response()
            }
        }
    }
}

/// Handler

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
enum BlackjackWsRequestData {
    Hit(HitWsRequestData),
    Stand(StandWsRequestData),
}

#[derive(Debug, Deserialize)]
struct HitWsRequestData {
    token: uuid::Uuid,
}

#[derive(Debug, Deserialize)]
struct StandWsRequestData {
    token: uuid::Uuid,
}

#[derive(Debug, Serialize)]
struct WsCardData {
    rank: u8,
    suit: u8,
}

impl From<Card> for WsCardData {
    fn from(source: Card) -> Self {
        Self {
            rank: source.value(),
            suit: source.face(),
        }
    }
}

impl From<GameOutcome> for WsGameOutcomeData {
    fn from(source: GameOutcome) -> Self {
        match source {
            GameOutcome::PlayerBlackjack => WsGameOutcomeData::PlayerBlackjack,
            GameOutcome::PlayerWin => WsGameOutcomeData::PlayerWin,
            GameOutcome::PlayerBust => WsGameOutcomeData::PlayerBust,
            GameOutcome::DealerBlackjack => WsGameOutcomeData::DealerBlackjack,
            GameOutcome::DealerWin => WsGameOutcomeData::DealerWin,
            GameOutcome::DealerBust => WsGameOutcomeData::DealerBust,
            GameOutcome::Standoff => WsGameOutcomeData::Standoff,
        }
    }
}

#[derive(Debug, Serialize)]
enum WsGameOutcomeData {
    PlayerBlackjack,
    PlayerWin,
    PlayerBust,
    DealerBlackjack,
    DealerWin,
    DealerBust,
    Standoff,
}

#[derive(Debug, Serialize)]
struct HitWsResponseData {
    card: WsCardData,
    outcome: Option<WsGameOutcomeData>,
}

impl From<HitWsRequestData> for HitRequest {
    fn from(source: HitWsRequestData) -> Self {
        Self::new(source.token)
    }
}

impl From<StandWsRequestData> for StandRequest {
    fn from(source: StandWsRequestData) -> Self {
        Self::new(source.token)
    }
}

#[derive(Debug, Serialize)]
struct StandWsResponseData {
    outcome: WsGameOutcomeData,
}

async fn hit<BS: BlackjackService>(
    req: HitWsRequestData,
    state: AppState<BS>,
) -> Result<HitWsResponseData, ApiError> {
    let domain_req = req.into();
    state
        .blackjack_service
        .hit(&domain_req)
        .await
        .map_err(ApiError::from)
        .map(|(card, outcome)| {
            HitWsResponseData {
                card: card.into(),
                outcome: outcome.map(|x| x.into()),
            }
        })
}

async fn stand<BS: BlackjackService>(
    req: StandWsRequestData,
    state: AppState<BS>,
) -> Result<StandWsResponseData, ApiError> {
    let domain_req = req.into();
    state
        .blackjack_service
        .stand(&domain_req)
        .await
        .map_err(ApiError::from)
        .map(|outcome| {
            StandWsResponseData {
                outcome: outcome.into(),
            }
        })
}

async fn handler<BS: BlackjackService>(
    mut socket: WebSocket,
    state: AppState<BS>,
) {
    while let Some(msg) = socket.recv().await {
        if let Ok(msg) = msg {
            match &msg {
                Message::Text(utf8_bytes) => {
                    use BlackjackWsRequestData::*;
                    match serde_json::from_str::<BlackjackWsRequestData>(utf8_bytes.as_str()) {
                        Ok(msg) => {
                            tracing::debug!("Request: {:?}", msg);
                            let response = match msg {
                                Hit(req) => {
									let response = hit::<BS>(req, state.clone()).await;
                                    tracing::debug!("Response: {:?}", response);
                                    serde_json::to_string(&response).unwrap()
                                },
                                Stand(req) => {
									let response = stand::<BS>(req, state.clone()).await;
                                    tracing::debug!("Response: {:?}", response);
                                    serde_json::to_string(&response).unwrap()
								},
                            };
                            tracing::debug!("Sending: {:?}", response);
                            if socket.send(response.into()).await.is_err() {
                                return;
                            }
                        }
                        Err(err) => tracing::debug!("{}", err),
                    };
                }
                _ => tracing::debug!("invalid message"),
            };

            // if socket.send(msg).await.is_err() {
            //     // client disconnected
            //     return;
            // }
        } else {
            tracing::debug!("invalid message");
            // client disconnected
            return;
        };
    }
}

pub async fn blackjack_ws_handler<BS: BlackjackService>(
	ws: WebSocketUpgrade,
	State(app_state): State<AppState<BS>>
) -> Response {
	ws.on_upgrade(move |ws| handler(ws, app_state))
}
