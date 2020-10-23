use crate::gameplay::gameplay_models::{GameDiscardRequest, PlayerGameStateResponse};
use crate::models::{GameSessions, Players, WebSocketResponse};
use crate::Result;
use game::gameplay::PlayerMove;
use game::{GameMove, GameState};
use std::collections::HashMap;
use warp::http::StatusCode;
use warp::reply::{json, Reply};
use warp::ws::Message;

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

pub async fn draw_card_handler(
    params: HashMap<String, String>,
    sessions: GameSessions,
) -> Result<impl Reply> {
    println!("Draw Card Request");
    if let Some(game) = sessions.write().await.get_mut(&params["game-id"]) {
        PlayerMove::handle_move(&PlayerMove::Draw, &mut game.state).unwrap();
        Ok(StatusCode::OK)
    } else {
        Err(warp::reject::not_found())
    }
}

pub async fn discard_handler(
    request: GameDiscardRequest,
    players: Players,
    sessions: GameSessions,
) -> Result<impl Reply> {
    println!("End Turn Request");
    if let Some(game) = sessions.write().await.get_mut(&request.game_id) {
        PlayerMove::handle_move(&PlayerMove::Discard(request.card_index), &mut game.state).unwrap();
        game.state.end_turn();
        players.read().await.iter().for_each(|(_, player)| {
            if let Some(sender) = &player.sender {
                sender
                    .send(Ok(Message::text(
                        serde_json::to_string(&WebSocketResponse {
                            response_type: "GameState".into(),
                            data: {},
                        })
                        .unwrap(),
                    )))
                    .unwrap();
            }
        });
        Ok(StatusCode::OK)
    } else {
        Err(warp::reject::not_found())
    }
}
