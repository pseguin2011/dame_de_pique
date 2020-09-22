use card_game_engine::deck::{Card, CardValue, Deck, DeckType};
use card_game_engine::error::{CardGameError, DefaultCardGameError};
use card_game_engine::player::Player;
use card_game_engine::game::{DefaultPlayerMoves, Game, GameBuilder, GameRunner};
use crate::partners::{Partners, WhoOpened};
use crate::error::DameDePiqueError;

use std::collections::HashMap;

#[derive(Clone)]
pub enum PlayerMove {
    Draw,
    Open(Vec<Card>),
    AddPoints(Vec<Card>),
    TakeDiscardPile,
    Discard(usize),
}

pub struct DameDePiqueGameBuilder;

pub struct DameDePiqueGameRunner<'a> {
    pub player_move: PlayerMove,
    pub partners: &'a mut Partners,
}

// impl DameDePiqueGameBuilder {

//     /// ### Purpose
//     /// Determines if a player can open based on the situation in the game, with partners, deck value, wild cards,
//     /// etc...
//     /// 
//     /// ### Arguments
//     /// * `player_cards` - the player's cards to verify
//     /// 
//     /// ### Returns
//     /// Whether the player can open with the cards in hand, if the top card of the deck is needed, or if the player cannot open
//     /// at all. 
//     /// 
//     /// ### TODO
//     /// This method is missing the addition of wild cards, opening with the top card on the discard pile,
//     /// opening if the partner has already opened (Only requiring 1 set of 3 in that case)
//     fn player_can_open<'b> (&self, player_cards: &[Card], with_discard_deck: bool, _partner: &'b Player) -> bool {
//         let mut card_sets: HashMap<CardValue, usize> = HashMap::new();
//         for card in player_cards {
//             match card_sets.get_mut(&card.value) {
//                 Some(v) =>  *v += 1,
//                 None => { card_sets.insert(card.value, 1); },
//             }
//         }

//         if with_discard_deck {
//             if let Some(top_card) = self.deck.peek_top_discarded_card() {
//                 match card_sets.get_mut(&top_card.value) {
//                     Some(v) =>  *v += 1,
//                     None => { card_sets.insert(top_card.value.clone(), 1); },
//                 }
//             }
//         }

//         // two and jokers cannot be counted towards an opening move
//         card_sets.remove(&CardValue::Two);
//         card_sets.remove(&CardValue::Joker);

//         let full_set_iterator = card_sets.iter();
//         let sets_of_three_or_more = full_set_iterator
//             .filter(|(_, &v)| v >= 3)
//             .count();
        

//         // player can open without top card
//         if sets_of_three_or_more >= 3 {
//             return true;
//         }
//         if let Some(card_value) = self.deck.peek_top_discarded_card() {
//             if let (2, Some(2)) = (sets_of_three_or_more, card_sets.get(&card_value.value)) {
//                 return true;
//             }
//         }

//         false
//     }
    
// }

impl <'a> DameDePiqueGameRunner <'a> {
    fn hand_can_open(&self, who_opened: WhoOpened, hand: &[Card]) -> bool {
        let mut cards: HashMap<CardValue, usize> = HashMap::new();
        for card in hand.iter() {
            match cards.get_mut(&card.value) {
                Some(v) => *v += 1,
                None => {
                    if card.value != CardValue::Joker { 
                        cards.insert(card.value, 1);
                    }
                },
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
            },
            WhoOpened::Nobody => {
                std::cmp::min(twos, doubles) + triples >= 3
            },
        }
    }

    fn can_add_points(&self, partners: &Partners, game: &Game) -> bool {
        match partners.who_opened(game.turn) {
            WhoOpened::Nobody | WhoOpened::Partner => false,
            WhoOpened::Me | WhoOpened::Both => true,
        }
    }
}

impl GameBuilder<DameDePiqueError> for DameDePiqueGameBuilder {
    fn initialize_game() -> Result<Game, DameDePiqueError> {
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
        
        Ok(Game {
            players,
            deck,
            turn: 0,
        })
    }
}

impl <'a> GameRunner<DameDePiqueError> for DameDePiqueGameRunner<'a> {
    fn player_move(&mut self, game: &mut Game) -> Result<(), DameDePiqueError> {
        match self.player_move.clone() {
            PlayerMove::Draw => if let Err(e) = game.player_move(DefaultPlayerMoves::Draw) {
                return Err(e.into());
            }
            PlayerMove::Discard(c) => if let Err(e) = game.player_move(DefaultPlayerMoves::Discard(c)) {
                return Err(e.into());
            },
            PlayerMove::TakeDiscardPile => {

                let mut player_hand_with_discard = game.players[game.turn].hand.clone();
                if let Some(card) = game.deck.peek_top_discarded_card() {
                    player_hand_with_discard.push(card.clone());
                }
                
                if !self.hand_can_open(
                    self.partners.who_opened(game.turn),
                    &player_hand_with_discard,
                ) { return Err(DameDePiqueError::InvalidDiscardOpeningHand(game.turn)); }
                for card in game.deck.take_discard_pile().drain(..) {
                    game.players[game.turn].add_card_to_hand(card);
                }
            },
            PlayerMove::Open(cards) => {
                if !self.hand_can_open(
                    self.partners.who_opened(game.turn),
                    &cards
                ) { return Err(DameDePiqueError::InvalidOpeningHand(game.turn));}
                self.partners.add_points(cards);
                self.partners.update_status(game.turn);
            },
            PlayerMove::AddPoints(cards) => {
                if !self.can_add_points(self.partners, game) { return Err(DameDePiqueError::PlayerCantAddPoints(game.turn)); }
                if !self.partners.are_valid_points(&cards) { return Err(DameDePiqueError::InvalidPoints); }
                self.partners.add_points(cards);
            },
        }
        Ok(())
    }
}

mod tests {
    use card_game_engine::deck::{Card, CardValue, CardSuit, Deck, DeckType};
    use card_game_engine::player::Player;
    use card_game_engine::game::{Game, GameBuilder};

    use crate::game::{ DameDePiqueGameRunner, PlayerMove};
    use crate::partners::{ Partners, WhoOpened };

    #[test]
    fn hand_open() {
        let mut partners = Partners::new(0, 1);
        
        let game = DameDePiqueGameRunner{
            player_move: PlayerMove::Draw,
            partners: &mut partners,
        };

        let mut hand = vec![
            Card {value: CardValue::Ace, suit: CardSuit::Clubs},
            Card {value: CardValue::Ace, suit: CardSuit::Spades},
            Card {value: CardValue::Ace, suit: CardSuit::Diamonds},
        ];

        assert!(game.hand_can_open(WhoOpened::Partner, &hand));
        assert!(!game.hand_can_open(WhoOpened::Nobody, &hand));

        hand.extend_from_slice(&[
            Card {value: CardValue::Three, suit: CardSuit::Clubs},
            Card {value: CardValue::Three, suit: CardSuit::Spades},
            Card {value: CardValue::Three, suit: CardSuit::Diamonds},

            Card {value: CardValue::Eight, suit: CardSuit::Clubs},
            Card {value: CardValue::Eight, suit: CardSuit::Spades},
            Card {value: CardValue::Eight, suit: CardSuit::Diamonds},
        ]);


        assert!(game.hand_can_open(WhoOpened::Partner, &hand));
        assert!(game.hand_can_open(WhoOpened::Nobody, &hand));
    }

    
}