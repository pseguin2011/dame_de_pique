use crate::models::{GameSession, GameSessions, Player, Players, WebSocketResponse};
use futures::{FutureExt, StreamExt};
use game::Game;
use serde::Deserialize;
use serde::Serialize;
use serde_json::from_str;
use tokio::sync::mpsc;
use warp::ws::{Message, WebSocket};

pub async fn client_connection(
    ws: WebSocket,
    player_id: String,
    clients: Players,
    games: GameSessions,
    mut client: Player,
) {
    let (client_ws_sender, mut client_ws_rcv) = ws.split();
    let (client_sender, client_rcv) = mpsc::unbounded_channel();

    tokio::task::spawn(client_rcv.forward(client_ws_sender).map(|result| {
        if let Err(e) = result {
            eprintln!("error sending websocket msg: {}", e);
        }
    }));

    client.sender = Some(client_sender);
    clients.write().await.insert(player_id.clone(), client);

    println!("{} connected", player_id);

    while let Some(result) = client_ws_rcv.next().await {
        let msg = match result {
            Ok(msg) => msg,
            Err(e) => {
                eprintln!(
                    "error receiving ws message for id: {}): {}",
                    player_id.clone(),
                    e
                );
                break;
            }
        };
        client_msg(&player_id, msg, &clients).await;
    }

    println!("{} disconnected", player_id);

    let mut players = clients.write().await;
    let player = players.get(&player_id).unwrap();
    let game_id_option = &player.inner.game_session_id;
    if let Some(game_id) = game_id_option {
        if let Some(game) = games.write().await.get_mut(game_id) {
            game.inner.players.remove(&player.inner.username);
            players.remove(&player_id);
            game.inner.players.iter().for_each(|player_id: &String| {
                if let Some(player) = players.get(player_id) {
                    if let Some(sender) = &player.sender {
                        sender
                            .send(Ok(Message::text(
                                serde_json::to_string(&WebSocketResponse {
                                    data: game.inner.clone(),
                                    response_type: "GameSession".into(),
                                })
                                .unwrap(),
                            )))
                            .unwrap();
                    }
                }
            });
        }
    }
}

async fn client_msg(id: &str, msg: Message, clients: &Players) {
    println!("received message from {}: {:?}", id, msg);
    let message = match msg.to_str() {
        Ok(v) => v,
        Err(_) => return,
    };

    if message == "ping" || message == "ping\n" {
        return;
    }
}
