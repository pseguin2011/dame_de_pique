use std::collections::HashMap;
use std::convert::Infallible;
use std::sync::Arc;
use tokio::sync::RwLock;
use warp::{http::Method, Filter, Rejection};
mod handler;
mod models;
mod ws;

use models::{GameSessions, Players};
type Result<T> = std::result::Result<T, Rejection>;

#[tokio::main]
async fn main() {
    let sessions: GameSessions = Arc::new(RwLock::new(HashMap::new()));
    let players: Players = Arc::new(RwLock::new(HashMap::new()));

    let health_route = warp::path!("health").and_then(handler::health_handler);

    let player_register = warp::path("player-register");
    let player_register_route = player_register
        .and(warp::post())
        .and(warp::body::json())
        .and(with_players(players.clone()))
        .and_then(handler::register_player_handler)
        .or(player_register
            .and(warp::delete())
            .and(warp::path::param())
            .and(with_players(players.clone()))
            .and(with_game_sessions(sessions.clone()))
            .and_then(handler::unregister_player_handler));

    let start_game_route = warp::path!("game-start")
        .and(warp::body::json())
        .and(with_players(players.clone()))
        .and(with_game_sessions(sessions.clone()))
        .and_then(handler::start_game_handler);

    let lobby_route = warp::path!("lobby")
        .and(with_game_sessions(sessions.clone()))
        .and_then(handler::get_lobby);

    let game_register = warp::path("game-register");
    let game_register_route = game_register
        .and(warp::post())
        .and(warp::body::json())
        .and(with_players(players.clone()))
        .and(with_game_sessions(sessions.clone()))
        .and_then(handler::register_game_handler);

    let ws_route = warp::path("ws")
        .and(warp::ws())
        .and(warp::path::param())
        .and(with_players(players.clone()))
        .and_then(handler::ws_handler);

    let cors = warp::cors()
        .allow_any_origin()
        .allow_header("content-type")
        .allow_methods(&[Method::GET, Method::POST, Method::DELETE]);
    let routes = health_route
        .or(player_register_route)
        .or(game_register_route)
        .or(lobby_route)
        .or(ws_route)
        .or(start_game_route)
        .with(cors);

    warp::serve(routes).run(([127, 0, 0, 1], 8000)).await;
    // warp::serve(routes).run(([10, 0, 0, 153], 8000)).await;
}

fn with_game_sessions(
    game_sessions: GameSessions,
) -> impl Filter<Extract = (GameSessions,), Error = Infallible> + Clone {
    warp::any().map(move || game_sessions.clone())
}

fn with_players(players: Players) -> impl Filter<Extract = (Players,), Error = Infallible> + Clone {
    warp::any().map(move || players.clone())
}
