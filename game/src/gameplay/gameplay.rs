use crate::error::DameDePiqueError;
use crate::partners::{Partners, WhoOpened};
use card_game_engine::game::{DefaultGameState, Game, GameBuilder, GameRunner, GameState};
use card_game_engine::models::deck::{Card, CardValue, Deck, DeckType};
use card_game_engine::models::player::Player;
use card_game_engine::moves::{DefaultMove, GameMove};

use std::collections::HashMap;

#[derive(Clone)]
pub enum PlayerMove {
    Draw,
    Open(Vec<Card>),
    AddPoints(Vec<Card>),
    TakeDiscardPile,
    Discard(usize),
}

#[derive(Clone)]
pub struct DDPState {
    pub default_state: DefaultGameState,
    pub partners: Vec<Partners>,
}

impl GameState for DDPState {
    fn end_turn(&mut self) {
        self.default_state.end_turn();
    }
}

impl DDPState {
    pub fn get_partners_from_player(&mut self, player: usize) -> &mut Partners {
        for partner in &mut self.partners {
            if partner.get_partner(player).is_some() {
                return partner;
            }
        }
        panic!()
    }
}

pub struct DameDePiqueGameBuilder;

impl PlayerMove {
    fn hand_can_open(who_opened: WhoOpened, hand: &[Card]) -> bool {
        let mut cards: HashMap<CardValue, usize> = HashMap::new();
        for card in hand.iter() {
            match cards.get_mut(&card.value) {
                Some(v) => *v += 1,
                None => {
                    if card.value != CardValue::Joker {
                        cards.insert(card.value, 1);
                    }
                }
            }
        }
        let twos = match cards.remove(&CardValue::Two) {
            Some(n) => n,
            None => 0,
        };
        let triples = cards.iter().filter(|(_, &v)| v >= 3).count();
        let doubles = cards.iter().filter(|(_, &v)| v == 2).count();

        match who_opened {
            WhoOpened::Both | WhoOpened::Me | WhoOpened::Partner => {
                std::cmp::min(twos, doubles) + triples >= 1
            }
            WhoOpened::Nobody => std::cmp::min(twos, doubles) + triples >= 3,
        }
    }

    fn can_add_points(turn: usize, game: &mut DDPState) -> bool {
        match game.get_partners_from_player(turn).who_opened(turn) {
            WhoOpened::Nobody | WhoOpened::Partner => false,
            WhoOpened::Me | WhoOpened::Both => true,
        }
    }
}

impl GameBuilder for DameDePiqueGameBuilder {
    type E = DameDePiqueError;
    type G = Game<DDPState>;
    fn initialize_game() -> Result<Self::G, Self::E> {
        let mut deck = Deck::new(DeckType::WithJokers);
        deck.extend(Deck::new(DeckType::WithJokers));
        deck.shuffle();

        let mut players = Vec::new();

        for i in 0..4 {
            players.push(Player::new(format!("Player {}", i), deck.draw_cards(13)?));
        }

        if let Some(top_card) = deck.draw_card() {
            deck.discard_card(top_card);
        }

        let state = DDPState {
            default_state: DefaultGameState {
                players,
                deck,
                turn: 0,
            },
            partners: vec![Partners::new(0, 2), Partners::new(1, 3)],
        };
        Ok(Game { state })
    }
}

impl GameMove<DDPState> for PlayerMove {
    type E = DameDePiqueError;
    fn handle_move(&self, game: &mut DDPState) -> Result<(), Self::E> {
        match self {
            PlayerMove::Draw => {
                if let Err(e) =
                    DefaultMove::handle_move(&DefaultMove::Draw, &mut game.default_state)
                {
                    return Err(e.into());
                }
            }
            PlayerMove::Discard(c) => {
                if let Err(e) =
                    DefaultMove::handle_move(&DefaultMove::Discard(*c), &mut game.default_state)
                {
                    return Err(e.into());
                }
            }
            PlayerMove::TakeDiscardPile => {
                let turn = game.default_state.turn;

                let mut player_hand_with_discard = game.default_state.players
                    [game.default_state.turn]
                    .hand
                    .clone();
                if let Some(card) = game.default_state.deck.peek_top_discarded_card() {
                    player_hand_with_discard.push(card.clone());
                }

                if !PlayerMove::hand_can_open(
                    game.get_partners_from_player(turn).who_opened(turn),
                    &player_hand_with_discard,
                ) {
                    return Err(DameDePiqueError::InvalidDiscardOpeningHand(
                        game.default_state.turn,
                    ));
                }
                for card in game.default_state.deck.take_discard_pile().drain(..) {
                    game.default_state.players[game.default_state.turn].add_card_to_hand(card);
                }
            }
            PlayerMove::Open(cards) => {
                let turn = game.default_state.turn;
                if !PlayerMove::hand_can_open(
                    game.get_partners_from_player(turn).who_opened(turn),
                    &cards,
                ) {
                    return Err(DameDePiqueError::InvalidOpeningHand(
                        game.default_state.turn,
                    ));
                }
                game.get_partners_from_player(turn)
                    .add_points(cards.to_vec());
                game.get_partners_from_player(turn).update_status(turn);
            }
            PlayerMove::AddPoints(cards) => {
                let turn = game.default_state.turn;
                if !PlayerMove::can_add_points(turn, game) {
                    return Err(DameDePiqueError::PlayerCantAddPoints(
                        game.default_state.turn,
                    ));
                }
                if !game
                    .get_partners_from_player(game.default_state.turn)
                    .are_valid_points(&cards)
                {
                    return Err(DameDePiqueError::InvalidPoints);
                }
                game.get_partners_from_player(game.default_state.turn)
                    .add_points(cards.to_vec());
            }
        }
        Ok(())
    }
}

mod tests {
    use card_game_engine::game::GameBuilder;
    use card_game_engine::models::deck::{Card, CardSuit, CardValue};

    use crate::gameplay::PlayerMove;
    use crate::partners::{Partners, WhoOpened};

    #[test]
    fn hand_open() {
        let mut partners = Partners::new(0, 1);

        let mut hand = vec![
            Card {
                value: CardValue::Ace,
                suit: CardSuit::Clubs,
            },
            Card {
                value: CardValue::Ace,
                suit: CardSuit::Spades,
            },
            Card {
                value: CardValue::Ace,
                suit: CardSuit::Diamonds,
            },
        ];

        assert!(PlayerMove::hand_can_open(WhoOpened::Partner, &hand));
        assert!(!PlayerMove::hand_can_open(WhoOpened::Nobody, &hand));

        hand.extend_from_slice(&[
            Card {
                value: CardValue::Three,
                suit: CardSuit::Clubs,
            },
            Card {
                value: CardValue::Three,
                suit: CardSuit::Spades,
            },
            Card {
                value: CardValue::Three,
                suit: CardSuit::Diamonds,
            },
            Card {
                value: CardValue::Eight,
                suit: CardSuit::Clubs,
            },
            Card {
                value: CardValue::Eight,
                suit: CardSuit::Spades,
            },
            Card {
                value: CardValue::Eight,
                suit: CardSuit::Diamonds,
            },
        ]);

        assert!(PlayerMove::hand_can_open(WhoOpened::Partner, &hand));
        assert!(PlayerMove::hand_can_open(WhoOpened::Nobody, &hand));
    }
}
