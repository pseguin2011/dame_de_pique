pub mod deck;
pub mod player;
pub mod error;
mod gameplay;

use deck::Deck;
use deck::DeckType;
use player::Player;
use gameplay::Game;

pub use gameplay::Turn;

fn main() {
    initialize_game();
}

fn initialize_game() -> Result<(), error::CardGameError> {
    let mut game = Game::new()?;
    println!("{:?}",game.players[0]);
    game.player_move(0, Turn::Draw);
    println!();
    println!("{:?}",game.players[0]);
    println!();
    game.player_move(0, Turn::Discard(1));
    println!("{:?}",game.players[0]);
    Ok(())
}