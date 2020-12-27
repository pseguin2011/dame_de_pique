use crate::gameplay::gameplay_models::{
    GameDiscardRequest, PlayerAddPointsRequest, PlayerGameStateResponse, PlayerOpenRequest,
    PlayerPickupDiscardRequest,
};
use crate::models::{GameSession, GameSessions, Players, WebSocketResponse};
use crate::Result;
use game::gameplay::DameDePiqueGameBuilder;
use game::gameplay::PlayerMove;
use game::rules::{GameRules, GameStatus};
use game::state::GameState;
use game::Game;
use serde::Serialize;
use std::collections::HashMap;
use warp::http::StatusCode;
use warp::reply::{json, Reply};
use warp::ws::Message;

type DameDePiqueGame = Game<DameDePiqueGameBuilder, PlayerMove>;

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
        DameDePiqueGame::game_action(PlayerMove::Draw, &mut game.state).unwrap();
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
        handle_game_status(
            DameDePiqueGame::game_action(PlayerMove::Discard(request.card_index), &mut game.state)
                .unwrap(),
            game,
            players.clone(),
        )
        .await;
        PlayerMove::end_turn(&mut game.state);

        let message = WebSocketResponse {
            response_type: "GameState".into(),
            data: {},
        };
        send_message_to_players(message, game, players).await;
        Ok(StatusCode::OK)
    } else {
        Err(warp::reject::not_found())
    }
}

pub async fn player_open_handler(
    request: PlayerOpenRequest,
    players: Players,
    sessions: GameSessions,
) -> Result<impl Reply> {
    println!("Player Open Request");
    if let Some(game) = sessions.write().await.get_mut(&request.game_id) {
        let hand = game.state.default_state.players[game.state.default_state.turn]
            .hand
            .iter();
        let cards = hand
            .enumerate()
            .filter(|(i, _c)| request.card_indices.contains(i))
            .map(|c| c.1.clone())
            .collect::<Vec<game::models::Card>>();
        match DameDePiqueGame::game_action(PlayerMove::Open(cards), &mut game.state) {
            Ok(GameStatus::Active) => {
                for i in request.card_indices.iter().rev() {
                    game.state.default_state.players[game.state.default_state.turn]
                        .hand
                        .remove(*i);
                }
            }
            Err(_) => return Err(warp::reject::reject()),
            Ok(other_status) => handle_game_status(other_status, game, players.clone()).await,
        }

        let message = WebSocketResponse {
            response_type: "GameState".into(),
            data: {},
        };
        send_message_to_players(message, game, players).await;
        Ok(StatusCode::OK)
    } else {
        Err(warp::reject::not_found())
    }
}

pub async fn player_add_points_handler(
    request: PlayerAddPointsRequest,
    players: Players,
    sessions: GameSessions,
) -> Result<impl Reply> {
    println!("Player Open Request");
    if let Some(game) = sessions.write().await.get_mut(&request.game_id) {
        let hand = game.state.default_state.players[game.state.default_state.turn]
            .hand
            .iter();
        let cards = hand
            .enumerate()
            .filter(|(i, _c)| request.card_indices.contains(i))
            .map(|c| c.1.clone())
            .collect::<Vec<game::models::Card>>();
        match DameDePiqueGame::game_action(PlayerMove::AddPoints(cards), &mut game.state) {
            Ok(GameStatus::Active) => {
                for i in request.card_indices.iter().rev() {
                    game.state.default_state.players[game.state.default_state.turn]
                        .hand
                        .remove(*i);
                }
            }
            Err(_) => return Err(warp::reject::reject()),
            Ok(other_status) => handle_game_status(other_status, game, players.clone()).await,
        }

        let message = WebSocketResponse {
            response_type: "GameState".into(),
            data: {},
        };
        send_message_to_players(message, game, players).await;
        Ok(StatusCode::OK)
    } else {
        Err(warp::reject::not_found())
    }
}

pub async fn player_pickup_discard_handler(
    request: PlayerPickupDiscardRequest,
    players: Players,
    sessions: GameSessions,
) -> Result<impl Reply> {
    println!("Player Pickup Discard Request");
    if let Some(game) = sessions.write().await.get_mut(&request.game_id) {
        let mut cards = Vec::new();
        for i in request.card_indices.iter().rev() {
            cards.push(
                game.state.default_state.players[game.state.default_state.turn]
                    .hand
                    .remove(*i),
            );
        }

        match DameDePiqueGame::game_action(
            PlayerMove::TakeDiscardPile(cards.clone()),
            &mut game.state,
        ) {
            Err(_) => {
                cards.iter().for_each(|c| {
                    game.state.default_state.players[game.state.default_state.turn]
                        .hand
                        .push(c.clone())
                });
                game.state.default_state.players[game.state.default_state.turn]
                    .hand
                    .sort();
                return Err(warp::reject::reject());
            }
            Ok(GameStatus::Active) => {}
            Ok(other_status) => handle_game_status(other_status, game, players.clone()).await,
        }

        let message = WebSocketResponse {
            response_type: "GameState".into(),
            data: {},
        };
        send_message_to_players(message, game, players).await;

        Ok(StatusCode::OK)
    } else {
        Err(warp::reject::not_found())
    }
}

pub async fn handle_game_status<'a>(
    status: GameStatus,
    session: &'a mut GameSession,
    players: Players,
) {
    match status {
        GameStatus::GameOver => {
            let message = WebSocketResponse {
                response_type: "EndGame".into(),
                data: {},
            };
            send_message_to_players(message, session, players).await;
        }
        GameStatus::RoundOver => {
            let message = WebSocketResponse {
                response_type: "EndGame".into(),
                data: {},
            };
            send_message_to_players(message, session, players).await;
        }
        _ => {}
    }
}

pub async fn send_message_to_players<T: Serialize>(
    message: WebSocketResponse<T>,
    session: &mut GameSession,
    players: Players,
) {
    let players = players.read().await;

    session.inner.players.iter().for_each(|player_id: &String| {
        if let Some(player) = players.get(player_id) {
            if let Some(sender) = &player.sender {
                sender
                    .send(Ok(Message::text(serde_json::to_string(&message).unwrap())))
                    .unwrap();
            }
        }
    });
}
