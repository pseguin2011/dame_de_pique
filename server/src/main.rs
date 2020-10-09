use std::collections::HashMap;
use std::convert::Infallible;
use std::sync::Arc;
use tokio::sync::{RwLock};
use warp::{http::StatusCode, http::response::Response, http::Method, Filter, Rejection};
mod handler;
mod models;

use models::{GameSessions, Players};

type Result<T> = std::result::Result<T, Rejection>;

#[tokio::main]
async fn main() {
    // let cors = warp::cors()
    // .allow_origin("http://localhost:19006")
    // .allow_origin("http://10.0.0.153:19006/")
    // .allow_methods(vec!["GET", "POST", "DELETE"]);

    let sessions: GameSessions = Arc::new(RwLock::new(HashMap::new()));
    let players: Players = Arc::new(RwLock::new(HashMap::new()));

    let health_route = warp::path!("health").and_then(handler::health_handler);

    let player_register = warp::path("player-register");
    let player_register_route = player_register
        .and(warp::post())
        .and(warp::body::json())
        .and(with_players(players.clone()))
        .and_then(handler::register_player_handler);

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

    let cors = warp::cors()
        .allow_any_origin()
        .allow_header("content-type")
        .allow_methods(&[Method::GET, Method::POST, Method::DELETE]);

    let routes = health_route
        .or(player_register_route)
        .or(game_register_route)
        .or(lobby_route)
        .with(cors);

    warp::serve(routes).run(([127, 0, 0, 1], 8000)).await;
}

fn with_game_sessions(game_sessions: GameSessions) -> impl Filter<Extract = (GameSessions,), Error = Infallible> + Clone {
    warp::any().map(move || game_sessions.clone())
}

fn with_players(players: Players) -> impl Filter<Extract = (Players,), Error = Infallible> + Clone {
    warp::any().map(move || players.clone())
}