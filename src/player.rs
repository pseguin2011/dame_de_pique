use crate::deck::Card;

#[derive(Debug)]
pub struct Player {
    name: String,
    hand: Vec<Card>,
}

impl Player {
    pub fn new(name: String, hand: Vec<Card>) -> Self {
        Player {
            name,
            hand,
        }
    }

    pub fn add_card_to_hand(&mut self, card: Card) {
        self.hand.push(card);
    }

    pub fn play_card_from_hand(&mut self, index: usize) -> Card {
        self.hand.remove(index)
    }
}