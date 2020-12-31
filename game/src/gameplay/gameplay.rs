use crate::error::DameDePiqueError;
use crate::partners::{Partners, WhoOpened};
use card_game_engine::builder::GameBuilder;
use card_game_engine::models::deck::{Card, CardSuit, CardValue, Deck, DeckType};
use card_game_engine::models::player::Player;
use card_game_engine::rules::{DefaultMove, GameRules, GameStatus};
use card_game_engine::state::GameState;

use std::collections::HashMap;

const GAME_POINT_TOTAL: u16 = 1000;

#[derive(Clone)]
pub enum PlayerMove {
    Draw,
    Open(Vec<Card>),
    AddPoints(Vec<Card>),
    TakeDiscardPile(Vec<Card>),
    Discard(usize),
}

#[derive(Clone)]
pub struct DDPState {
    pub default_state: GameState,
    pub partners: Vec<Partners>,
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
    /// Verifies that the provided hand can open
    ///
    /// ## Rules
    /// * If neither player on the team has opened,
    ///     3 sets of 3 cards are required to open
    /// * If the partner has opened
    ///     1 sets of 3 cards is required to open
    /// * Sets excludes Two's and Jokers
    /// * Two's are wild and can be added to an
    ///     incomplete set to complete it
    /// * A player may not open twice
    ///
    /// ## Arguments
    /// `who_opened` - An enum representing which player(s) on the
    ///                 team have opened
    /// `hand` - The cards being verified
    ///
    /// ## Returns
    /// A boolean of whether the hand can open or not
    fn hand_can_open(who_opened: WhoOpened, hand: &[Card]) -> bool {
        let mut cards: HashMap<CardValue, usize> = HashMap::new();
        for card in hand.iter() {
            match cards.get_mut(&card.value) {
                Some(v) => *v += 1,
                None => {
                    // Jokers are not counted toward an opening hand
                    if card.value != CardValue::Joker {
                        cards.insert(card.value, 1);
                    }
                }
            }
        }

        // Two's are not counted toward sets of 3 but are considered wild
        let mut twos = match cards.remove(&CardValue::Two) {
            Some(n) => n,
            None => 0,
        } as i32;

        // Complete sets
        let triples = cards.iter().filter(|(_, &v)| v == 3).count();
        // Sets of 2 (assuming one wild)
        let doubles = cards.iter().filter(|(_, &v)| v == 2).count();
        // Sets of 1 (assuming two wilds)
        let singles = cards.iter().filter(|(_, &v)| v == 1).count();

        match who_opened {
            WhoOpened::Both | WhoOpened::Me => false,
            WhoOpened::Partner => {
                // Verifying that the amount of Two's provided complete the set
                // in the provided hand
                twos -= (singles * 2) as i32;
                if twos != (doubles as i32) {
                    return false;
                }
                singles + doubles + triples == 1
            }
            WhoOpened::Nobody => {
                // Verifying that the amount of Two's provided complete every set
                // in the provided hand
                twos -= (singles * 2) as i32;
                if twos != (doubles as i32) {
                    return false;
                }
                singles + doubles + triples == 3
            }
        }
    }

    fn can_add_points(turn: usize, game: &mut DDPState) -> bool {
        match game.get_partners_from_player(turn).who_opened(turn) {
            WhoOpened::Nobody | WhoOpened::Partner => false,
            WhoOpened::Me | WhoOpened::Both => true,
        }
    }

    /// Returns whether the top card is even a card that is allowed to be picked up
    /// e.g. Not a 2, Joker or a card in the player's shared points
    fn player_can_pickup_top_discard(turn: usize, game: &mut DDPState) -> bool {
        let top_card = game.default_state.deck.peek_top_discarded_card().cloned();
        let who_opened = &game.get_partners_from_player(turn).who_opened(turn);
        let points_deck = &game.get_partners_from_player(turn).points_deck;

        if let Some(top_card) = top_card {
            let can_pickup = top_card.value != CardValue::Joker
                && top_card.value != CardValue::Two
                && (!points_deck.contains_key(&top_card.value)
                    || match who_opened {
                        WhoOpened::Nobody | WhoOpened::Partner => true,
                        _ => false,
                    });
            println!(
                "The player is allowed to pickup the top discard: {}",
                can_pickup
            );
            can_pickup
        } else {
            false
        }
    }
}

impl GameBuilder for DameDePiqueGameBuilder {
    type E = DameDePiqueError;
    type S = DDPState;
    fn initialize_game() -> Result<Self::S, Self::E> {
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
            default_state: GameState {
                players,
                deck,
                turn: 0,
            },
            partners: vec![Partners::new(0, 2), Partners::new(1, 3)],
        };
        Ok(state)
    }
}

impl GameRules<DDPState, DameDePiqueError> for PlayerMove {
    fn handle_move(&self, game: &mut DDPState) -> Result<GameStatus, DameDePiqueError> {
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
            PlayerMove::TakeDiscardPile(cards) => {
                // Rules: A user can pickup the discard pile at all times,
                // If the user hasn't opened, then the restrictions
                let turn = game.default_state.turn;
                let mut cards = cards.clone();

                // Verification that the player can't open before adding the top discarded
                // card
                if PlayerMove::hand_can_open(
                    game.get_partners_from_player(turn).who_opened(turn),
                    &cards,
                ) || !PlayerMove::player_can_pickup_top_discard(turn, game)
                {
                    println!(
                        "The user could already open or the top card was not valid to pickup."
                    );
                    return Err(DameDePiqueError::InvalidDiscardOpeningHand(
                        game.default_state.turn,
                    ));
                }

                if let Some(card) = game.default_state.deck.peek_top_discarded_card() {
                    cards.push(card.clone());
                }
                match game.get_partners_from_player(turn).who_opened(turn) {
                    WhoOpened::Nobody | WhoOpened::Partner => {
                        if !PlayerMove::hand_can_open(
                            game.get_partners_from_player(turn).who_opened(turn),
                            &cards,
                        ) {
                            println!(
                                "The hand wasn't valid to open with, {:?} : {:?}",
                                &cards,
                                game.get_partners_from_player(turn).who_opened(turn)
                            );
                            return Err(DameDePiqueError::InvalidDiscardOpeningHand(
                                game.default_state.turn,
                            ));
                        }
                    }
                    _ => {
                        if !PlayerMove::hand_can_open(WhoOpened::Partner, &cards) {
                            println!("The hand wasn't valid to pickup deck with, {:?}", &cards);
                            return Err(DameDePiqueError::InvalidDiscardOpeningHand(
                                game.default_state.turn,
                            ));
                        }
                    }
                }
                // Valid Opening hand to pickup the discard pile so we remove the top card of the dicard pile
                game.default_state.deck.pop_top_discarded_card();

                // Opening action
                game.get_partners_from_player(turn)
                    .add_points(cards.to_vec());
                game.get_partners_from_player(turn).update_status(turn);

                // Adding cards from discard pile to hand
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

        if Self::is_game_over(game) && Self::is_round_over(game) {
            Ok(GameStatus::GameOver)
        } else if Self::is_round_over(game) {
            for mut partner in game.partners.iter_mut() {
                partner.overall_points += partner.get_points_total() as i16;
                let player_1_points = Self::calculate_point_total(
                    game.default_state.players[partner.player_a_index]
                        .hand
                        .clone(),
                );
                let player_2_points = Self::calculate_point_total(
                    game.default_state.players[partner.player_b_index]
                        .hand
                        .clone(),
                );
                partner.overall_points -= player_1_points as i16;
                partner.overall_points -= player_2_points as i16;
            }

            Ok(GameStatus::RoundOver)
        } else {
            Ok(GameStatus::Active)
        }
    }

    fn is_game_over(state: &mut DDPState) -> bool {
        for partner in state.partners.iter() {
            if partner.get_points_total() >= GAME_POINT_TOTAL {
                return true;
            }
        }
        false
    }

    fn is_round_over(state: &mut DDPState) -> bool {
        for player in state.default_state.players.iter() {
            if player.hand.is_empty() {
                return true;
            }
        }
        false
    }

    fn end_turn(state: &mut DDPState) {
        DefaultMove::end_turn(&mut state.default_state);
    }
}

impl PlayerMove {
    pub fn calculate_point_total(hand: Vec<Card>) -> u16 {
        let mut total = 0;
        for v in hand.iter() {
            total += match v.value {
                CardValue::Ace => 15,
                CardValue::Two => 20,
                CardValue::Three
                | CardValue::Four
                | CardValue::Five
                | CardValue::Six
                | CardValue::Seven
                | CardValue::Eight
                | CardValue::Nine => 5,
                CardValue::Ten | CardValue::Jack | CardValue::King => 10,
                CardValue::Queen => match v.suit {
                    CardSuit::Clubs | CardSuit::Hearts | CardSuit::Diamonds => 10,
                    CardSuit::Spades => 100,
                    _ => 0,
                },
                CardValue::Joker => 50,
            };
        }
        total
    }
}

mod tests {
    #[allow(unused_imports)]
    use crate::gameplay::PlayerMove;
    #[allow(unused_imports)]
    use crate::partners::WhoOpened;
    #[allow(unused_imports)]
    use card_game_engine::builder::GameBuilder;
    #[allow(unused_imports)]
    use card_game_engine::models::deck::{Card, CardSuit, CardValue};

    #[test]
    fn hand_open_before_parner() {
        let mut hand = vec![
            Card {
                value: CardValue::Ace,
                suit: CardSuit::Clubs,
            };
            3
        ];

        assert!(!PlayerMove::hand_can_open(WhoOpened::Nobody, &hand));

        hand.extend_from_slice(&vec![
            Card {
                value: CardValue::Three,
                suit: CardSuit::Clubs,
            };
            3
        ]);
        hand.extend_from_slice(&vec![
            Card {
                value: CardValue::Eight,
                suit: CardSuit::Clubs,
            };
            3
        ]);

        assert!(PlayerMove::hand_can_open(WhoOpened::Nobody, &hand));

        // Remove eights
        hand.pop();
        hand.pop();
        hand.pop();

        // Replace with 3 Two's (both are wild)
        hand.extend_from_slice(&vec![
            Card {
                value: CardValue::Two,
                suit: CardSuit::Spades,
            };
            3
        ]);

        // Invalid opening hand, 3 twos must be independently
        assert!(!PlayerMove::hand_can_open(WhoOpened::Nobody, &hand));

        hand.pop();
        hand.pop();
        hand.push(Card {
            value: CardValue::Five,
            suit: CardSuit::Spades,
        });
        hand.push(Card {
            value: CardValue::Jack,
            suit: CardSuit::Spades,
        });

        // Invalid opening hand including
        assert!(!PlayerMove::hand_can_open(WhoOpened::Nobody, &hand));

        hand.clear();
        hand.extend_from_slice(&vec![
            Card {
                value: CardValue::Two,
                suit: CardSuit::Spades,
            };
            6
        ]);
        hand.push(Card {
            value: CardValue::Three,
            suit: CardSuit::Spades,
        });
        hand.push(Card {
            value: CardValue::Five,
            suit: CardSuit::Spades,
        });
        hand.push(Card {
            value: CardValue::Seven,
            suit: CardSuit::Spades,
        });
    }

    #[test]
    fn open_with_wilds_after_partner() {
        let mut hand = vec![
            Card {
                value: CardValue::Ace,
                suit: CardSuit::Clubs,
            },
            Card {
                value: CardValue::Two,
                suit: CardSuit::Spades,
            },
            Card {
                value: CardValue::Two,
                suit: CardSuit::Diamonds,
            },
        ];
        assert!(PlayerMove::hand_can_open(WhoOpened::Partner, &hand));

        hand.clear();
        hand.extend_from_slice(&[
            Card {
                value: CardValue::Ace,
                suit: CardSuit::Clubs,
            },
            Card {
                value: CardValue::Ace,
                suit: CardSuit::Spades,
            },
            Card {
                value: CardValue::Two,
                suit: CardSuit::Diamonds,
            },
        ]);
        assert!(PlayerMove::hand_can_open(WhoOpened::Partner, &hand));

        hand.clear();
        hand.extend_from_slice(&[
            Card {
                value: CardValue::Two,
                suit: CardSuit::Clubs,
            },
            Card {
                value: CardValue::Two,
                suit: CardSuit::Spades,
            },
            Card {
                value: CardValue::Two,
                suit: CardSuit::Diamonds,
            },
        ]);
        assert!(!PlayerMove::hand_can_open(WhoOpened::Partner, &hand));
    }

    #[test]
    fn opening_twice() {
        // valid hand
        let hand = [
            Card {
                value: CardValue::Four,
                suit: CardSuit::Clubs,
            },
            Card {
                value: CardValue::Four,
                suit: CardSuit::Spades,
            },
            Card {
                value: CardValue::Four,
                suit: CardSuit::Diamonds,
            },
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
        ];

        assert!(PlayerMove::hand_can_open(WhoOpened::Nobody, &hand));

        // A player cannot open twice
        assert!(!PlayerMove::hand_can_open(WhoOpened::Both, &hand));
        assert!(!PlayerMove::hand_can_open(WhoOpened::Me, &hand));
    }
}
