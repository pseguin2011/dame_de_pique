use crate::{Result};
use std::collections::HashSet;
use uuid::Uuid;
use warp::{http::StatusCode, reply::json, Reply};

use crate::models::{GameSessionListResponse, GameSession, GameSessions, Player, PlayerResponse, Players, RegisterGameRequest, RegisterPlayerRequest};

pub async fn register_game_handler(body: RegisterGameRequest, players: Players, sessions: GameSessions) -> Result<impl Reply> {
    let uuid = Uuid::new_v4().to_string();
    let url = format!("ws://127.0.0.1:8000/ws/{}", uuid);
    let game_session = register_game(body.game_identifier.clone(), url.clone(), body.player_username.clone(), players, sessions).await;
    Ok(json(&game_session))
}

async fn register_game(game_id: String, url: String, player_username: String, players: Players, sessions: GameSessions) -> GameSession {
    let mut sessions = sessions.write().await;
    if let Some(mut player) = players.write().await.get_mut(&player_username) {
        player.inner.game_session_id = Some(game_id.clone());
    } else {
        panic!("Player Doesn't exist");
    }

    let mut players = HashSet::new();
    players.insert(player_username);
    
    let game_session = GameSession {
        game_id: game_id.clone(),
        players,
        max_capacity: 4,
        url,
    };

    sessions.insert(
        game_id.clone(),
        game_session.clone(),
    );
    game_session
}

pub async fn get_lobby(sessions: GameSessions) -> Result<impl Reply> {
    let sessions = sessions.read().await;
    Ok(json(&GameSessionListResponse { games: sessions.values().cloned().collect() }))
}

pub async fn register_player_handler(body: RegisterPlayerRequest, players: Players) -> Result<impl Reply> {
    register_player(body.username, players).await;
    Ok(StatusCode::OK)
}

async fn register_player(username: String, players: Players) {
    let mut players = players.write().await;
    players.insert(
        username.clone(),
        Player {
            inner: PlayerResponse {
                username,
                game_session_id: None,
            },
            sender: None,
        },
    );
}

pub async fn health_handler() -> Result<impl Reply> {
    Ok(StatusCode::OK)
}

pub async fn unregister_player_handler(username: String, players: Players, sessions: GameSessions) -> Result<impl Reply> {
    let player = players.write().await.remove(&username);
    let mut removed_player = None;
    if let Some(player) = player {
        if let Some(game_session_id) = player.inner.game_session_id {
            if let Some(session) = sessions.write().await.get_mut(&game_session_id) {
                removed_player = Some(session.players.remove(&username));
            }
        }
    }
    Ok(json(&removed_player))
}