use card_game_engine::models::deck::{Card, CardValue, CardSuit, Deck, DeckType};
use card_game_engine::models::player::Player;
use card_game_engine::game::{Game, GameRunner, DefaultGameState};

use dame_de_pique::game::{ PlayerMove, DDPState, DameDePiqueGameBuilder};
use dame_de_pique::partners::{Partners};
use dame_de_pique::error::DameDePiqueError;

#[test]
fn player_b_open() -> Result<(), DameDePiqueError> {

    let deck = Deck::new(DeckType::WithJokers);
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

    let state = DDPState {
        default_state: DefaultGameState {
            players,
            deck,
            turn: 0,
        },
        partners: vec![Partners::new(0, 1)],
    };

    let mut game = Game { state };

    game.player_move(PlayerMove::Draw)?;

    let hand = game.state.default_state.players[0].hand.clone();
    assert!(game.player_move(PlayerMove::Open(hand)).is_err());
    game.end_turn();

    game.player_move(PlayerMove::Draw)?;
    let hand = game.state.default_state.players[1].hand.clone();
    assert!(game.player_move(PlayerMove::Open(hand)).is_ok());


    Ok(())
}

    #[test]
    fn player_a_open_with_discard() {
        let mut deck = Deck::new(DeckType::WithJokers);
        let players = vec![
            Player::new("Player 1", vec![
                Card {value: CardValue::Ace, suit: CardSuit::Spades},
                Card {value: CardValue::Ace, suit: CardSuit::Hearts},
                Card {value: CardValue::Ace, suit: CardSuit::Diamonds},

                Card {value: CardValue::Three, suit: CardSuit::Spades},
                Card {value: CardValue::Three, suit: CardSuit::Hearts},
                Card {value: CardValue::Three, suit: CardSuit::Diamonds},

                Card {value: CardValue::Ten, suit: CardSuit::Hearts},
                Card {value: CardValue::Ten, suit: CardSuit::Diamonds},
            ]),
        ];
        
        deck.discard_card(
            Card {value: CardValue::Ten, suit: CardSuit::Spades});

        let state = DDPState {
            default_state: DefaultGameState {
                players,
                deck,
                turn: 0,
            },
            partners: vec![Partners::new(0, 1)],
        };

        let mut game = Game { state };

        let hand = game.state.default_state.players[0].hand.clone();

        assert!(game.player_move(PlayerMove::Open(hand)).is_err());

        assert!(game.player_move(PlayerMove::TakeDiscardPile).is_ok());

        let hand = game.state.default_state.players[0].hand.clone();
        assert!(game.player_move(PlayerMove::Open(hand)).is_ok());
    }


    #[test]
    fn player_b_open_with_discard() {
        let mut deck = Deck::new(DeckType::WithJokers);
        let players = vec![
            Player::new("Player 1", vec![
                Card {value: CardValue::Seven, suit: CardSuit::Diamonds},
                Card {value: CardValue::Seven, suit: CardSuit::Hearts},
            ]),
            Player::new("Player 2", vec![
                Card {value: CardValue::Ace, suit: CardSuit::Spades},
                Card {value: CardValue::Ace, suit: CardSuit::Hearts},
                Card {value: CardValue::Ace, suit: CardSuit::Diamonds},
                Card {value: CardValue::Three, suit: CardSuit::Spades},
                Card {value: CardValue::Three, suit: CardSuit::Hearts},
                Card {value: CardValue::Three, suit: CardSuit::Diamonds},
                Card {value: CardValue::Four, suit: CardSuit::Spades},
                Card {value: CardValue::Four, suit: CardSuit::Hearts},
                Card {value: CardValue::Four, suit: CardSuit::Diamonds},
            ]),
        ];
        
        deck.discard_card(
            Card {value: CardValue::Seven, suit: CardSuit::Diamonds});

        let state = DDPState {
            default_state: DefaultGameState {
                players,
                deck,
                turn: 1,
            },
            partners: vec![Partners::new(0, 1)],
        };

        let mut game = Game { state };

        let hand = game.state.default_state.players[1].hand.clone();

        assert!(game.player_move(PlayerMove::Open(hand)).is_ok());
        game.end_turn();

        assert!(game.player_move(PlayerMove::TakeDiscardPile).is_ok());

        let hand = game.state.default_state.players[0].hand.clone();

        assert!(game.player_move(PlayerMove::Open(hand)).is_ok());
    }