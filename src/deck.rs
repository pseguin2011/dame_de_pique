use rand;
use crate::error::CardGameError;

pub const CARD_VALUE: [&str; 13] = [
    "Ace",
    "Two",
    "Three",
    "Four",
    "Five",
    "Six",
    "Seven",
    "Eight",
    "Nine",
    "Ten",
    "Jack",
    "Queen",
    "King",
];

pub const CARD_SUIT: [&str; 4] = [
    "Hearts",
    "Diamonds",
    "Spades",
    "Clubs"
];

pub enum DeckType {
    Normal,
    WithJokers,
}

#[derive(Debug)]
pub struct Card {
    value: String,
    suit: String,
}

#[derive(Debug)]
pub struct Deck {
    deck: Vec<Card>,
    discard_pile: Vec<Card>,
}

impl Deck {
    pub fn new(deck_type: DeckType) -> Deck {
        let mut deck: Vec<Card> = Vec::new();
        let discard_pile: Vec<Card> = Vec::new();
        for suit in CARD_SUIT.to_vec() {
            for value in CARD_VALUE.to_vec() {
                deck.push( Card { value: value.into(), suit: suit.into()});
            }
        }

        if let DeckType::WithJokers = deck_type {
            deck.push(Card {value: String::from("Joker"), suit: String::from("Color")});
            deck.push(Card {value: String::from("Joker"), suit: String::from("Black")});
        }
        
        Deck { deck, discard_pile }
    }

    pub fn extend(&mut self, new_deck: Deck) {
        self.deck.extend(new_deck.deck);
    }

    pub fn shuffle(&mut self) {
        for _ in 0..1000 {
            let index_a = rand::random::<usize>() % self.deck.len();
            let index_b = rand::random::<usize>() % self.deck.len();
            self.deck.swap(index_a, index_b);
        }
    }

    /// Cycles through the amount of requested cards and returns the top `n` cards
    /// 
    /// ## Arguments
    /// `amount` - the number of cards drawn
    /// 
    /// ## Returns
    /// The `n` cards drawn from the deck
    pub fn draw_cards(&mut self, amount: usize) -> Result<Vec<Card>, CardGameError> {
        if amount > self.deck.len() {
            return Err(CardGameError::IncorrectCardNumberRequest)
        }
        let mut cards = Vec::new();
        for _ in 0..amount {
            if let Some(card) = self.draw_card() {
                cards.push(card);
            }
        }
        Ok(cards)
    }

    pub fn draw_card(&mut self) -> Option<Card> {
        self.deck.pop()
    }

    pub fn discard_card(&mut self, card: Card) {
        self.discard_pile.push(card);
    }
}