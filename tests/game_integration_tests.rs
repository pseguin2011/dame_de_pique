use card_game_engine::deck::{Card, CardValue, CardSuit, Deck, DeckType};
use card_game_engine::player::Player;
use card_game_engine::game::{Game};
use card_game_engine::error::CardGameError;

use dame_de_pique::game::{ DameDePiqueGameRunner, PlayerMove };
use dame_de_pique::partners::{Partners};
use dame_de_pique::error::DameDePiqueError;

#[test]
fn player_b_open() -> Result<(), DameDePiqueError> {

    let players = vec![
        Player::new("Player 1", vec![
            Card {value: CardValue::Seven, suit: CardSuit::Clubs},
            Card {value: CardValue::Seven, suit: CardSuit::Spades},
            Card {value: CardValue::Seven, suit: CardSuit::Diamonds}
        ]),
        Player::new("Player 2", vec![
            Card {value: CardValue::Ace, suit: CardSuit::Clubs},
            Card {value: CardValue::Ace, suit: CardSuit::Spades},
            Card {value: CardValue::Ace, suit: CardSuit::Diamonds},

            Card {value: CardValue::Five, suit: CardSuit::Clubs},
            Card {value: CardValue::Five, suit: CardSuit::Spades},
            Card {value: CardValue::Five, suit: CardSuit::Diamonds},

            Card {value: CardValue::Four, suit: CardSuit::Clubs},
            Card {value: CardValue::Four, suit: CardSuit::Spades},
            Card {value: CardValue::Four, suit: CardSuit::Diamonds},
        ]),
    ];

    let mut game = Game {
        players,
        deck: Deck::new(DeckType::WithJokers),
        turn: 0,
    };

    let mut partners = Partners::new(0, 1);
    let runner = DameDePiqueGameRunner {
        player_move: PlayerMove::Draw,
        partners: &mut partners,
    };

    game.player_move(runner)?;

    let hand = game.players[0].hand.clone();
    let runner = DameDePiqueGameRunner {
        player_move: PlayerMove::Open(hand),
        partners: &mut partners,
    };
    assert!(game.player_move(runner).is_err());
    game.end_turn();

    let runner = DameDePiqueGameRunner {
        player_move: PlayerMove::Draw,
        partners: &mut partners,
    };

    game.player_move(runner)?;

    let hand = game.players[1].hand.clone();
    let runner = DameDePiqueGameRunner {
        player_move: PlayerMove::Open(hand),
        partners: &mut partners,
    };
    assert!(game.player_move(runner).is_ok());


    Ok(())
}

//     #[test]
//     fn player_a_open_with_discard() {
//         let mut deck = Deck::new(DeckType::WithJokers);
//         let players = [
//             Player::new("Player 1", vec![
//                 Card {value: CardValue::Ace, suit: CardSuit::Spades},
//                 Card {value: CardValue::Ace, suit: CardSuit::Hearts},
//                 Card {value: CardValue::Ace, suit: CardSuit::Diamonds},

//                 Card {value: CardValue::Three, suit: CardSuit::Spades},
//                 Card {value: CardValue::Three, suit: CardSuit::Hearts},
//                 Card {value: CardValue::Three, suit: CardSuit::Diamonds},

//                 Card {value: CardValue::Ten, suit: CardSuit::Hearts},
//                 Card {value: CardValue::Ten, suit: CardSuit::Diamonds},
//             ]),
//             Player::new("Player 2", vec![]),
//             Player::new("Player 3", vec![]),
//             Player::new("Player 4", vec![]),
//         ];
        
//         deck.discard_card(
//             Card {value: CardValue::Four, suit: CardSuit::Spades});

//         let partners = [
//             Partners::new(0, 1),
//             Partners::new(2,3),
//         ];
        
//         let game = Game {
//             players,
//             partners,
//             deck,
//         };

//         assert!(game.player_can_open(&game.players[0].hand, true, &game.players[1]));
//     }

//     #[test]
//     fn player_b_open_with_discard() {
//         let mut deck = Deck::new(DeckType::WithJokers);
//         let mut players = [
//             Player::new("Player 1", vec![
//                 Card {value: CardValue::Seven, suit: CardSuit::Diamonds},
//                 Card {value: CardValue::Seven, suit: CardSuit::Hearts},
//             ]),
//             Player::new("Player 2", vec![
//                 Card {value: CardValue::Ace, suit: CardSuit::Spades},
//                 Card {value: CardValue::Ace, suit: CardSuit::Hearts},
//                 Card {value: CardValue::Ace, suit: CardSuit::Diamonds},
//                 Card {value: CardValue::Three, suit: CardSuit::Spades},
//                 Card {value: CardValue::Three, suit: CardSuit::Hearts},
//                 Card {value: CardValue::Three, suit: CardSuit::Diamonds},
//                 Card {value: CardValue::Four, suit: CardSuit::Spades},
//                 Card {value: CardValue::Four, suit: CardSuit::Hearts},
//                 Card {value: CardValue::Four, suit: CardSuit::Diamonds},
//             ]),
//             Player::new("Player 3", vec![]),
//             Player::new("Player 4", vec![]),
//         ];
        
//         deck.discard_card(
//             Card {value: CardValue::Seven, suit: CardSuit::Diamonds});

//         let mut partners = [
//             Partners::new(0, 1),
//             Partners::new(2,3),
//         ];
        
//         partners[0].update_status(1);
//         partners[0].add_points(std::mem::take(&mut players[1].hand));

//         let game = Game {
//             players,
//             partners,
//             deck,
//         };

//         assert!(game.player_can_open(&game.players[0].hand, true, &game.players[1]));
//     }