use crate::deck::Card;

use std::collections::HashMap;

#[derive(Debug)]
pub struct Player {
    name: String,
    pub hand: Vec<Card>,
    pub has_opened: bool,
}

impl Player {
    pub fn new<S: Into<String>>(name: S, hand: Vec<Card>) -> Self {
        Player {
            name: name.into(),
            hand,
            has_opened: false,
        }
    }

    pub fn add_card_to_hand(&mut self, card: Card) {
        self.hand.push(card);
    }

    pub fn play_card_from_hand(&mut self, index: usize) -> Card {
        self.hand.remove(index)
    }
}

#[derive(Clone, Copy, Debug)]
pub enum TeamStatus {
    NoneOpen,
    PlayerAOpen,
    PlayerBOpen,
    BothOpen,
}

#[derive(Debug)]
pub struct Partners {
    player_a_index: usize,
    player_b_index: usize,
    points_deck: HashMap<String, Vec<Card>>,
    status: TeamStatus,
}

impl Partners {
    pub fn new(player_a_index: usize, player_b_index: usize) -> Self {
        Self {
            player_a_index,
            player_b_index,
            points_deck: HashMap::new(),
            status: TeamStatus::NoneOpen,
        }
    }

    pub fn add_points(&mut self, cards: Vec<Card>) {
        for card in cards {
            match self.points_deck.get_mut(&card.value) {
                Some(v) => v.push(card),
                None => { self.points_deck.insert(card.value.clone(), vec![card]); },
            }
        }
    }

    pub fn update_status(&mut self, player_opening: usize) {
        let status;
        if player_opening == self.player_a_index {
            status = TeamStatus::PlayerAOpen;
        } else if player_opening == self.player_b_index {
            status = TeamStatus::PlayerBOpen;
        } else {
            // invalid user opening here
            return;
        }

        match (status, &self.status) {
            (TeamStatus::PlayerAOpen, TeamStatus::PlayerBOpen) |
            (TeamStatus::PlayerBOpen, TeamStatus::PlayerAOpen) => {
                self.status = TeamStatus::BothOpen
            },
            (s, TeamStatus::NoneOpen) => {
                self.status = s;
            }
            _ => {},
        }
    }

    pub fn get_status(&self) -> TeamStatus {
        self.status
    }

    pub fn get_partner(&self, player_index: usize) -> Option<usize> {
        if player_index == self.player_a_index {
            Some(self.player_b_index)
        } else if player_index == self.player_b_index {
            Some(self.player_a_index)
        } else {
            None
        }
    }
}


#[test]
fn test_status() {
    let mut partners = Partners::new(0,1);
    partners.update_status(0);
    match partners.get_status() {
        TeamStatus::PlayerAOpen => {},
        _ => {
            panic!("Assertion failed, got {:?}, expected {:?}", partners.get_status(), TeamStatus::PlayerAOpen);
        },
    }

    partners.update_status(1);
    match partners.get_status() {
        TeamStatus::BothOpen => {},
        _ => {
            panic!("Assertion failed, got {:?}, expected {:?}", partners.get_status(), TeamStatus::BothOpen);
        },
    }

    let mut partners = Partners::new(0,1);
    partners.update_status(1);
    match partners.get_status() {
        TeamStatus::PlayerBOpen => {},
        _ => {
            panic!("Assertion failed, got {:?}, expected {:?}", partners.get_status(), TeamStatus::PlayerBOpen);
        },
    }

    partners.update_status(1);
    match partners.get_status() {
        TeamStatus::PlayerBOpen => {},
        _ => {
            panic!("Assertion failed, got {:?}, expected {:?}", partners.get_status(), TeamStatus::PlayerBOpen);
        },
    }

    partners.update_status(0);
    match partners.get_status() {
        TeamStatus::BothOpen => {},
        _ => {
            panic!("Assertion failed, got {:?}, expected {:?}", partners.get_status(), TeamStatus::BothOpen);
        },
    }

}