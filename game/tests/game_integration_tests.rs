extern crate game;

use card_game_engine::game::Game;
use card_game_engine::models::deck::{Card, CardSuit, CardValue, Deck, DeckType};
use card_game_engine::models::player::Player;
use card_game_engine::rules::GameStatus;
use card_game_engine::state::GameState;
use game::gameplay::{DDPState, DameDePiqueGameBuilder, PlayerMove};

// use dame_de_pique::game::{ PlayerMove, DDPState};
// use dame_de_pique::partners::{Partners};
use game::error::DameDePiqueError;
type DameDePiqueGame = Game<DameDePiqueGameBuilder, PlayerMove>;

#[test]
fn calculate_total() -> Result<(), DameDePiqueError> {
    let mut game_state = DameDePiqueGame::new_game()?;
    game_state.default_state.players[0].hand.clear();
    game_state.default_state.players[0].hand.push(Card {
        suit: CardSuit::Clubs,
        value: CardValue::Ace,
    });
    assert_eq!(
        GameStatus::RoundOver,
        DameDePiqueGame::game_action(PlayerMove::Discard(0), &mut game_state)?
    );
    let player_1_points =
        PlayerMove::calculate_point_total(game_state.default_state.players[0].hand.clone()) as i16;
    let player_2_points =
        PlayerMove::calculate_point_total(game_state.default_state.players[2].hand.clone()) as i16;
    assert_eq!(
        game_state.partners[0].overall_points,
        game_state.partners[0].get_points_total() as i16 - player_1_points - player_2_points
    );
    Ok(())
}

// #[test]
// fn player_b_open() -> Result<(), DameDePiqueError> {

//     let deck = Deck::new(DeckType::WithJokers);
//     let players = vec![
//         Player::new("Player 1", vec![
//             Card {value: CardValue::Seven, suit: CardSuit::Clubs},
//             Card {value: CardValue::Seven, suit: CardSuit::Spades},
//             Card {value: CardValue::Seven, suit: CardSuit::Diamonds}
//         ]),
//         Player::new("Player 2", vec![
//             Card {value: CardValue::Ace, suit: CardSuit::Clubs},
//             Card {value: CardValue::Ace, suit: CardSuit::Spades},
//             Card {value: CardValue::Ace, suit: CardSuit::Diamonds},

//             Card {value: CardValue::Five, suit: CardSuit::Clubs},
//             Card {value: CardValue::Five, suit: CardSuit::Spades},
//             Card {value: CardValue::Five, suit: CardSuit::Diamonds},

//             Card {value: CardValue::Four, suit: CardSuit::Clubs},
//             Card {value: CardValue::Four, suit: CardSuit::Spades},
//             Card {value: CardValue::Four, suit: CardSuit::Diamonds},
//         ]),
//     ];

//     let state = DDPState {
//         default_state: DefaultGameState {
//             players,
//             deck,
//             turn: 0,
//         },
//         partners: vec![Partners::new(0, 1)],
//     };

//     let mut game = Game { state };

//     game.player_move(PlayerMove::Draw)?;

//     let hand = game.state.default_state.players[0].hand.clone();
//     assert!(game.player_move(PlayerMove::Open(hand)).is_err());
//     game.end_turn();

//     game.player_move(PlayerMove::Draw)?;
//     let hand = game.state.default_state.players[1].hand.clone();
//     assert!(game.player_move(PlayerMove::Open(hand)).is_ok());

//     Ok(())
// }

//     #[test]
//     fn player_a_open_with_discard() {
//         let mut deck = Deck::new(DeckType::WithJokers);
//         let players = vec![
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
//         ];

//         deck.discard_card(
//             Card {value: CardValue::Ten, suit: CardSuit::Spades});

//         let state = DDPState {
//             default_state: DefaultGameState {
//                 players,
//                 deck,
//                 turn: 0,
//             },
//             partners: vec![Partners::new(0, 1)],
//         };

//         let mut game = Game { state };

//         let hand = game.state.default_state.players[0].hand.clone();

//         assert!(game.player_move(PlayerMove::Open(hand)).is_err());

//         assert!(game.player_move(PlayerMove::TakeDiscardPile).is_ok());

//         let hand = game.state.default_state.players[0].hand.clone();
//         assert!(game.player_move(PlayerMove::Open(hand)).is_ok());
//     }

//     #[test]
//     fn player_b_open_with_discard() {
//         let mut deck = Deck::new(DeckType::WithJokers);
//         let players = vec![
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
//         ];

//         deck.discard_card(
//             Card {value: CardValue::Seven, suit: CardSuit::Diamonds});

//         let state = DDPState {
//             default_state: DefaultGameState {
//                 players,
//                 deck,
//                 turn: 1,
//             },
//             partners: vec![Partners::new(0, 1)],
//         };

//         let mut game = Game { state };

//         let hand = game.state.default_state.players[1].hand.clone();

//         assert!(game.player_move(PlayerMove::Open(hand)).is_ok());
//         game.end_turn();

//         assert!(game.player_move(PlayerMove::TakeDiscardPile).is_ok());

//         let hand = game.state.default_state.players[0].hand.clone();

//         assert!(game.player_move(PlayerMove::Open(hand)).is_ok());
//     }
