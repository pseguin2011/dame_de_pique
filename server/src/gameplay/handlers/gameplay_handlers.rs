use crate::gameplay::gameplay_models::PlayerGameStateResponse;
use crate::models::GameSessions;
use crate::Result;
use std::collections::HashMap;
use warp::reply::{json, Reply};

pub async fn get_game_state_handler(
    params: HashMap<String, String>,
    sessions: GameSessions,
) -> Result<impl Reply> {
    println!("Gameplay Request");
    if let Some(game) = sessions.read().await.get(&params["game-id"]) {
        let player = &params["player"].parse::<usize>().unwrap();
        let mut response: PlayerGameStateResponse = game.clone().state.into();
        response.player_hand = game.state.default_state.players[*player]
            .hand
            .iter()
            .map(|v| crate::gameplay::gameplay_models::Card::from(v.clone()))
            .collect();
        Ok(json(&response))
    } else {
        Err(warp::reject::not_found())
    }
}
