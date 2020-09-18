#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
use std::sync::Mutex;

pub mod game;
use rocket::State;

use game::Game;

pub use game::{Turn, DrawOption};

use card_game_engine::error::CardGameError;

fn main() {
    initialize_game().unwrap();
    rocket::ignite().manage(Mutex::new(Game::new().unwrap())).mount("/", routes![game]).launch();
}

fn initialize_game() -> Result<(), CardGameError> {
    let mut game = Game::new()?;
    println!("{:?}",game.players[0]);
    game.player_move(0, Turn::Draw(DrawOption::DrawFromDeck));
    println!();
    println!("{:?}",game.players[0]);
    println!();
    game.player_move(0, Turn::Discard(1));
    println!("{:?}",game.players[0]);
    Ok(())
}

#[get("/game")]
fn game(game: State<Mutex<Game>>) -> String {
    format!("{:?}", game)
}

// #[post("/player/move/{player_id}/{move_id}")]
// fn player_move<'r>(mut game: State<Mutex<Game>>, player_id: usize, move_id: usize) -> String {
//     match move_id {
//         1 | 2 | 3 | 4=> {
//             game.inner().lock().unwrap().player_move(player_id, Turn::Draw(DrawOption::DrawFromDeck));
//         },

//         // format!("{:?}", game.inner().lock().unwrap().players[0])
//         _ => "404 Move not fount".to_string()
//     }
// }