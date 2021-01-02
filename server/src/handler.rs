use std::collections::HashSet;
use warp::http::StatusCode;
use warp::reply::{json, Reply};
use warp::ws::Message;

use game::gameplay::{DameDePiqueGameBuilder, PlayerMove};
use game::Game;

use crate::config;
use crate::models::{
    GameResponse, GameSession, GameSessionListResponse, GameSessions, Player, PlayerExists,
    PlayerResponse, Players, RegisterGameRequest, RegisterPlayerRequest, StartGameRequest,
    StartGameResponse, TooManyPlayers, WebSocketResponse,
};
use crate::Result;

type DameDePiqueGame = Game<DameDePiqueGameBuilder, PlayerMove>;

/// Handler for game registration
///
/// ## Purpose
/// Adds a game session to the list of games in the lobby
///
/// ## Arguments
/// `body` - the game being registered
/// `players` - persistent collection of players
/// `sessions` - persistent collection of game sessions
pub async fn register_game_handler(
    body: RegisterGameRequest,
    players: Players,
    sessions: GameSessions,
) -> Result<impl Reply> {
    println!("Registering game: {:?}", body);
    let game_session = register_game(
        body.game_identifier.clone(),
        body.player_username.clone(),
        players,
        sessions,
    )
    .await;
    Ok(json(&game_session?))
}

/// Adds a game to the collection of game sessions
///
/// ## Arguments
/// `game_id` - a unique game identifier
/// `url` - the game url request when all players are ready to play
/// `player_username` - the player creating the game session
/// `players` - persistent collection of players
/// `sessions` - persistent collection of the game sessions
async fn register_game(
    game_id: String,
    player_username: String,
    players: Players,
    sessions: GameSessions,
) -> Result<GameResponse> {
    let mut sessions = sessions.write().await;
    if let Some(mut player) = players.write().await.get_mut(&player_username) {
        player.inner.game_session_id = Some(game_id.clone());
    } else {
        // Only registered players can create games
        return Err(warp::reject::not_found());
    }

    match sessions.get_mut(&game_id) {
        Some(session) => {
            if session.inner.players.contains(&player_username) {
                return Ok(session.inner.clone());
            }
            if session.inner.players.len() >= 4 {
                return Err(warp::reject::custom(TooManyPlayers));
            }
            session.inner.players.insert(player_username);
            players.read().await.iter().for_each(|(_, player)| {
                if let Some(sender) = &player.sender {
                    sender
                        .send(Ok(Message::text(
                            serde_json::to_string(&WebSocketResponse {
                                response_type: "GameSession".into(),
                                data: session.inner.clone(),
                            })
                            .unwrap(),
                        )))
                        .unwrap();
                }
            });

            Ok(session.inner.clone())
        }
        None => {
            let mut players = HashSet::new();
            players.insert(player_username);

            let game_response = GameResponse {
                game_id: game_id.clone(),
                players,
            };
            let game_session = GameSession {
                is_active: false,
                inner: game_response.clone(),
                state: DameDePiqueGame::new_game().unwrap(),
            };

            sessions.insert(game_id.clone(), game_session);
            Ok(game_response)
        }
    }
}

/// Handler for the list of games in lobby
///
/// ## Purpose
/// Returns the list of active game sessions
///
/// ## Arguments
/// `sessions` - persistent collection of game sessions
pub async fn get_lobby(sessions: GameSessions) -> Result<impl Reply> {
    let sessions = sessions.read().await;
    Ok(json(&GameSessionListResponse {
        games: sessions.values().cloned().map(|v| v.inner).collect(),
    }))
}

/// Handler for players registering to the game
///
/// ## Purpose
/// Adds the player to the list of players in the lobby
///
/// ## Arguments
/// `body` - the player username being registered
/// `players` - persistent collection of players
pub async fn register_player_handler(
    body: RegisterPlayerRequest,
    players: Players,
) -> Result<impl Reply> {
    println!("Registering player: {:?}", body);
    let player_response = register_player(body.username, players).await?;
    Ok(json(&player_response))
}

/// Adds the player to the collection of players
///
/// ## Arguments
/// `username` - the player username being registered
/// `players` - persistent collection of players
async fn register_player(username: String, players: Players) -> Result<PlayerResponse> {
    let mut players = players.write().await;
    if players.contains_key(&username) {
        return Err(warp::reject::custom(PlayerExists));
    }

    let config = config::load_config();

    let player_response = PlayerResponse {
        username: username.clone(),
        game_session_id: None,
        websocket_url: format!(
            "ws://{}:{}/ws/{}",
            config.websocket_host, config.port, username
        ),
    };
    players.insert(
        username.clone(),
        Player {
            inner: player_response.clone(),
            sender: None,
        },
    );

    Ok(player_response)
}

pub async fn start_game_handler(
    body: StartGameRequest,
    players: Players,
    sessions: GameSessions,
) -> Result<impl Reply> {
    println!("Starting Game {}", body.game_id);

    let mut sessions = sessions.write().await;
    let mut session = sessions.get_mut(&body.game_id).unwrap();

    let team_1_points = session.state.partners[0].overall_points;
    let team_2_points = session.state.partners[1].overall_points;

    session.state = DameDePiqueGame::new_game().unwrap();

    session.state.partners[0].overall_points = team_1_points;
    session.state.partners[1].overall_points = team_2_points;

    let players = players.read().await;
    let response_type = if session.is_active {
        "GameState"
    } else {
        "StartGameResponse"
    };
    session.is_active = true;
    session.inner.players.iter().for_each(|player_id: &String| {
        if let Some(player) = players.get(player_id) {
            if let Some(sender) = &player.sender {
                sender
                    .send(Ok(Message::text(
                        serde_json::to_string(&WebSocketResponse {
                            response_type: response_type.into(),
                            data: StartGameResponse {},
                        })
                        .unwrap(),
                    )))
                    .unwrap();
            }
        }
    });

    Ok(StatusCode::OK)
}

pub async fn ws_handler(
    ws: warp::ws::Ws,
    player_id: String,
    players: Players,
    games: GameSessions,
) -> Result<impl Reply> {
    println!("Websocket Request for: user {}", player_id);
    let player = players.read().await.get(&player_id).cloned();
    match player {
        Some(c) => Ok(ws.on_upgrade(move |socket| {
            crate::ws::client_connection(socket, player_id, players, games, c)
        })),
        None => Err(warp::reject::not_found()),
    }
}

pub async fn health_handler() -> Result<impl Reply> {
    Ok(StatusCode::OK)
}

pub async fn unregister_player_handler(
    username: String,
    players: Players,
    sessions: GameSessions,
) -> Result<impl Reply> {
    let player = players.write().await.remove(&username);
    let mut removed_player = None;
    if let Some(player) = player {
        if let Some(game_session_id) = player.inner.game_session_id {
            if let Some(session) = sessions.write().await.get_mut(&game_session_id) {
                removed_player = Some(session.inner.players.remove(&username));
            }
        }
    }
    Ok(json(&removed_player))
}
