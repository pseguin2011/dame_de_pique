pub mod deck;
pub mod player;
pub mod error;
mod gameplay;

use gameplay::Game;

pub use gameplay::{Turn, DrawOption};

fn main() {
    initialize_game().unwrap();
}

fn initialize_game() -> Result<(), error::CardGameError> {
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