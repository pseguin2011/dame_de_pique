use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use tokio::sync::{mpsc, RwLock};
use warp::ws::Message;

pub type GameSessions = Arc<RwLock<HashMap<String, GameSession>>>;
pub type Players = Arc<RwLock<HashMap<String, Player>>>;

#[derive(Debug, Clone)]
pub struct Player {
    pub sender: Option<mpsc::UnboundedSender<std::result::Result<Message, warp::Error>>>,
    pub inner: PlayerResponse,
}

#[derive(Clone)]
pub struct GameSession {
    pub inner: GameResponse,
    pub state: game::gameplay::DDPState,
    pub is_active: bool,
}

#[derive(Clone, Debug, Serialize)]
pub struct GameResponse {
    pub game_id: String,
    pub players: HashSet<String>,
}

#[derive(Clone, Debug, Serialize)]
pub struct PlayerResponse {
    pub username: String,
    pub game_session_id: Option<String>,
    pub websocket_url: String,
}

#[derive(Deserialize, Debug)]
pub struct RegisterPlayerRequest {
    pub username: String,
}

#[derive(Deserialize, Debug)]
pub struct RegisterGameRequest {
    pub game_identifier: String,
    pub player_username: String,
}

#[derive(Serialize, Debug)]
pub struct GameSessionListResponse {
    pub games: Vec<GameResponse>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct StartGameRequest {
    pub game_id: String,
}

#[derive(Serialize, Debug)]
pub struct StartGameResponse;

#[derive(Serialize, Debug, Clone)]
pub struct WebSocketResponse<S: Serialize> {
    pub response_type: String,
    pub data: S,
}

/// Rejection messages
#[derive(Debug)]
pub struct TooManyPlayers;
impl warp::reject::Reject for TooManyPlayers {}

#[derive(Debug)]
pub struct PlayerExists;
impl warp::reject::Reject for PlayerExists {}
