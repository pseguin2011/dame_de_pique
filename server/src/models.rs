use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use tokio::sync::{mpsc, RwLock};
use serde::{Deserialize, Serialize};
use warp::{ws::Message};

pub type GameSessions = Arc<RwLock<HashMap<String, GameSession>>>;
pub type Players = Arc<RwLock<HashMap<String, Player>>>;

#[derive(Debug, Clone)]
pub struct Player {
    pub sender: Option<mpsc::UnboundedSender<std::result::Result<Message, warp::Error>>>,
    pub inner: PlayerResponse,
}

#[derive(Clone, Debug, Serialize)]
pub struct GameSession {
    pub game_id: String,
    // list of usernames for players
    pub players: HashSet<String>,
    pub max_capacity: usize,
    pub url: String,
}

#[derive(Clone, Debug, Serialize)]
pub struct PlayerResponse {
    pub username: String,
    pub game_session_id: Option<String>,
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
    pub games: Vec<GameSession>,
}