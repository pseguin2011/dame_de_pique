use card_game_engine::deck::{Card, CardValue, CardSuit, Deck, DeckType};
use card_game_engine::player::Player;
use card_game_engine::game::{Game};

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

        let mut partners = Partners::new(0, 1);
        
        let mut game = Game {
            players,
            deck,
            turn: 0,
        };

        let hand = game.players[0].hand.clone();
        let runner = DameDePiqueGameRunner{
            player_move: PlayerMove::Open(hand),
            partners: &mut partners,
        };
        assert!(game.player_move(runner).is_err());

        let runner = DameDePiqueGameRunner{
            player_move: PlayerMove::TakeDiscardPile,
            partners: &mut partners,
        };
        assert!(game.player_move(runner).is_ok());

        let hand = game.players[0].hand.clone();
        let runner = DameDePiqueGameRunner{
            player_move: PlayerMove::Open(hand),
            partners: &mut partners,
        };
        assert!(game.player_move(runner).is_ok());
    }

    #[test]
    fn player_b_open_with_discard() {
        let mut deck = Deck::new(DeckType::WithJokers);
        let mut players = vec![
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

        let mut partners = Partners::new(0, 1);

        let mut game = Game {
            players,
            deck,
            turn: 1,
        };

        let hand = game.players[1].hand.clone();
        let runner = DameDePiqueGameRunner {
            player_move: PlayerMove::Open(hand),
            partners: &mut partners,
        };

        assert!(game.player_move(runner).is_ok());
        game.end_turn();

        let runner = DameDePiqueGameRunner {
            player_move: PlayerMove::TakeDiscardPile,
            partners: &mut partners,
        };

        assert!(game.player_move(runner).is_ok());

        let hand = game.players[0].hand.clone();
        let runner = DameDePiqueGameRunner {
            player_move: PlayerMove::Open(hand),
            partners: &mut partners,
        };

        assert!(game.player_move(runner).is_ok());
    }