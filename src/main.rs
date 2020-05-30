pub mod deck;
pub mod player;
pub mod error;

use deck::Deck;
use deck::DeckType;
use player::Player;

fn main() {
    initialize_game();
}

fn initialize_game() -> Result<(), error::CardGameError> {
    let mut deck = Deck::new(DeckType::WithJokers);
    //two decks needed for this game
    deck.extend(Deck::new(DeckType::WithJokers));
    deck.shuffle();

    let mut players = Vec::with_capacity(4);
    for i in 1..5 {
        players.push(Player::new(format!("Player {}", i), deck.draw_cards(13).unwrap()));
    }
    
    let top_card = deck.draw_card();
    if let Some(card) = top_card {
        deck.discard_card(card);
    }
    
    println!("{:?}", deck);

    for player in players {
        println!("\n Player: {:?}", player);
    }

    Ok(())
}