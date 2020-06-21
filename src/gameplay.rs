use crate::deck::{Card, Deck, DeckType};
use crate::error::CardGameError;
use crate::player::Player;

use std::collections::HashMap;

pub enum Turn<'a> {
    Draw,
    PickUpDeck(Vec<&'a Card>),
    Open(Vec<Card>),
    Discard(usize),
}

pub enum OpenOption {
    CanOpen,
    CannotOpen,
}

#[derive(Debug)]
pub struct Game {
    pub players: [Player; 4],
    partners: [(usize, usize); 2],
    deck: Deck,
}

impl Game {
    pub fn new() -> Result<Self, CardGameError> {
        let mut deck = Deck::new(DeckType::WithJokers);
        //two decks needed for this game
        deck.extend(Deck::new(DeckType::WithJokers));
        deck.shuffle();
        
        let players = [
            Player::new("Player 1", deck.draw_cards(13)?),
            Player::new("Player 2", deck.draw_cards(13)?),
            Player::new("Player 3", deck.draw_cards(13)?),
            Player::new("Player 4", deck.draw_cards(13)?)
        ];
               
        if let Some(top_card) = deck.draw_card() {
            deck.discard_card(top_card);
        }

        let partners = [
            (0,1),
            (2,3),
        ];

        
        Ok(Game {
            players,
            partners,
            deck,
        })
    }

    pub fn player_move(&mut self, player: usize, player_move: Turn) {
        match player_move {
            Turn::Draw => {
                if let Some(card) = self.deck.draw_card() {
                    self.players[player].add_card_to_hand(card);
                }
            },
            Turn::PickUpDeck(_) => {
                unimplemented!();
            },
            Turn::Open(cards) => {
                match self.player_can_open(&cards) {
                    OpenOption::CanOpen => unimplemented!(),
                    OpenOption::CannotOpen => unimplemented!(),
                }
            },
            Turn::Discard(card_index) => {
                self.deck.discard_card(self.players[player].play_card_from_hand(card_index));
            },
        }
    }

    fn player_can_open(&self, player_cards: &[Card]) -> OpenOption {
        let mut card_sets: HashMap<String, usize> = HashMap::new();
        for card in player_cards {
            match card_sets.get_mut(&card.value) {
                Some(v) =>  *v += 1,
                None => { card_sets.insert(card.value.clone(), 1); },
            }
        }

        // two and jokers cannot be counted towards an opening move
        card_sets.remove("Two");
        card_sets.remove("Joker");

        let full_set_iterator = card_sets.iter();
        let sets_of_three_or_more = full_set_iterator
            .filter(|(_, &v)| v >= 3)
            .count();
        

        // player can open without top card
        if sets_of_three_or_more >= 3 {
            return OpenOption::CanOpen;
        }

        OpenOption::CannotOpen
    }
}