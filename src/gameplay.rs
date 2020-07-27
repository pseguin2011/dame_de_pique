use crate::deck::{Card, Deck, DeckType};
use crate::error::CardGameError;
use crate::player::{Partners, Player};

use std::collections::HashMap;


pub enum DrawOption {
    DrawFromDeck,
    PickUpDiscardDeckWithHand(Vec<Card>),
}

pub enum Turn {
    Draw(DrawOption),
    Open(Vec<Card>),
    AddPoints(Vec<Card>),
    Discard(usize),
}


pub enum OpenOption {
    CanOpen,
    CanOpenWithDiscardDeck,
    CannotOpen,
}

#[derive(Debug)]
pub struct Game {
    pub players: [Player; 4],
    partners: [Partners; 2],
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
            Player::new("Player 4", deck.draw_cards(13)?),
        ];
               
        if let Some(top_card) = deck.draw_card() {
            deck.discard_card(top_card);
        }

        let partners = [
            Partners::new(0, 1),
            Partners::new(2,3),
        ];
        
        Ok(Game {
            players,
            partners,
            deck,
        })
    }

    /// ### Purpose
    /// These are the options a use can take at any given turn, they are as follows:
    ///     Step 1: Draw card or Pick up the discard deck
    ///     Step 2: Open
    ///     Step 3: Discard
    /// 
    /// ### Arguments
    /// `player` - The index of the player performing the move
    /// `player_move` -
    pub fn player_move(&mut self, player: usize, player_move: Turn) {
        match player_move {
            Turn::Draw(DrawOption::DrawFromDeck) => {
                if let Some(card) = self.deck.draw_card() {
                    self.players[player].add_card_to_hand(card);
                }
            },
            Turn::Draw(DrawOption::PickUpDiscardDeckWithHand(hand)) => {
                if let (Some(player_index_of_partner), Some(partner_index)) = (self.player_index_of_partner(player), self.partners_index(player)) {
                    if self.player_can_open(&hand, true, &self.players[player_index_of_partner]) {
                        self.partners[partner_index].add_points(hand);
                        self.partners[partner_index].add_points(vec![self.deck.pop_top_discarded_card()]);
                        self.players[player].hand.extend(self.deck.take_discard_pile());
                    }
                }
            },
            Turn::Open(cards) => {
                if let (Some(player_index_of_partner), Some(partner_index)) = (self.player_index_of_partner(player), self.partners_index(player)) {
                    if self.player_can_open(&cards, false, &self.players[player_index_of_partner]) {
                        self.partners[partner_index].add_points(cards);
                    }
                } else {
                    panic!("Game was not initialized correctly, partner was not found!");
                }
            },
            Turn::AddPoints(cards) => {
                unimplemented!();
            },
            Turn::Discard(card_index) => {
                self.deck.discard_card(self.players[player].play_card_from_hand(card_index));
            },
        }
    }

    /// ### Purpose
    /// Determines if a player can open based on the situation in the game, with partners, deck value, wild cards,
    /// etc...
    /// 
    /// ### Arguments
    /// * `player_cards` - the player's cards to verify
    /// 
    /// ### Returns
    /// Whether the player can open with the cards in hand, if the top card of the deck is needed, or if the player cannot open
    /// at all. 
    /// 
    /// ### TODO
    /// This method is missing the addition of wild cards, opening with the top card on the discard pile,
    /// opening if the partner has already opened (Only requiring 1 set of 3 in that case)
    fn player_can_open<'b> (&self, player_cards: &[Card], with_discard_deck: bool, _partner: &'b Player) -> bool {
        let mut card_sets: HashMap<String, usize> = HashMap::new();
        for card in player_cards {
            match card_sets.get_mut(&card.value) {
                Some(v) =>  *v += 1,
                None => { card_sets.insert(card.value.clone(), 1); },
            }
        }

        if with_discard_deck {
            let top_card = self.deck.peek_top_discarded_card();
            match card_sets.get_mut(&top_card.value) {
                Some(v) =>  *v += 1,
                None => { card_sets.insert(top_card.value.clone(), 1); },
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
            return true;
        }

        if let (2, Some(2)) = (sets_of_three_or_more, card_sets.get(&self.deck.peek_top_discarded_card().value)) {
            return true;
        }

        false
    }

    /// Returns the player index of the partner
    /// 
    /// ## Purpose
    /// As the game is organized by players and partners, we must be able to find who the
    fn player_index_of_partner(&self, current_player: usize) -> Option<usize> {
        for partner in self.partners.iter() {
            if let Some(p) = partner.get_partner(current_player) {
                return Some(p);
            }
        }
        None
    }
    
    /// Returns the partners index of the provided player
    fn partners_index(&self, player: usize) -> Option<usize> {
        for i in 0..self.partners.len() {
            if self.partners[i].get_partner(player).is_some() {
                return Some(i);
            }
        }
        None
    }
    
}