use card_game_engine::models::deck::{Card, CardValue};
use std::collections::HashMap;

#[derive(Clone, Copy, Debug)]
enum TeamOpenStatus {
    None,
    PlayerA,
    PlayerB,
    Both,
}

pub enum WhoOpened {
    Both,
    Me,
    Partner,
    Nobody,
}

#[derive(Debug, Clone)]
pub struct Partners {
    player_a_index: usize,
    player_b_index: usize,
    pub points_deck: HashMap<CardValue, Vec<Card>>,
    status: TeamOpenStatus,
}

impl Partners {
    pub fn new(player_a_index: usize, player_b_index: usize) -> Self {
        Self {
            player_a_index,
            player_b_index,
            points_deck: HashMap::new(),
            status: TeamOpenStatus::None,
        }
    }

    pub fn add_points(&mut self, cards: Vec<Card>) {
        for card in cards {
            match self.points_deck.get_mut(&card.value) {
                Some(v) => v.push(card),
                None => {
                    self.points_deck.insert(card.value, vec![card]);
                }
            }
        }
    }

    pub fn update_status(&mut self, player_opening: usize) {
        let status;
        if player_opening == self.player_a_index {
            status = TeamOpenStatus::PlayerA;
        } else if player_opening == self.player_b_index {
            status = TeamOpenStatus::PlayerB;
        } else {
            // invalid user opening here
            return;
        }

        match (status, &self.status) {
            (TeamOpenStatus::PlayerA, TeamOpenStatus::PlayerB)
            | (TeamOpenStatus::PlayerB, TeamOpenStatus::PlayerA) => {
                self.status = TeamOpenStatus::Both
            }
            (s, TeamOpenStatus::None) => {
                self.status = s;
            }
            _ => {}
        }
    }

    fn get_status(&self) -> TeamOpenStatus {
        self.status
    }

    pub fn who_opened(&self, i: usize) -> WhoOpened {
        match self.get_status() {
            TeamOpenStatus::Both => WhoOpened::Both,
            TeamOpenStatus::PlayerA => {
                if self.player_a_index == i {
                    WhoOpened::Me
                } else {
                    WhoOpened::Partner
                }
            }
            TeamOpenStatus::PlayerB => {
                if self.player_b_index == i {
                    WhoOpened::Me
                } else {
                    WhoOpened::Partner
                }
            }
            TeamOpenStatus::None => WhoOpened::Nobody,
        }
    }

    pub fn are_valid_points(&self, hand: &[Card]) -> bool {
        let mut cards: HashMap<CardValue, usize> = HashMap::new();
        for card in hand {
            match cards.get_mut(&card.value) {
                Some(v) => *v += 1,
                None => {
                    if card.value != CardValue::Joker {
                        cards.insert(card.value, 1);
                    }
                }
            }
        }
        for (k, v) in cards {
            if self.points_deck.get(&k).is_none() && v <= 3 {
                return false;
            }
        }
        true
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
    let mut partners = Partners::new(0, 1);
    partners.update_status(0);
    match partners.get_status() {
        TeamOpenStatus::PlayerA => {}
        _ => {
            panic!(
                "Assertion failed, got {:?}, expected {:?}",
                partners.get_status(),
                TeamOpenStatus::PlayerA
            );
        }
    }

    partners.update_status(1);
    match partners.get_status() {
        TeamOpenStatus::Both => {}
        _ => {
            panic!(
                "Assertion failed, got {:?}, expected {:?}",
                partners.get_status(),
                TeamOpenStatus::Both
            );
        }
    }

    let mut partners = Partners::new(0, 1);
    partners.update_status(1);
    match partners.get_status() {
        TeamOpenStatus::PlayerB => {}
        _ => {
            panic!(
                "Assertion failed, got {:?}, expected {:?}",
                partners.get_status(),
                TeamOpenStatus::PlayerB
            );
        }
    }

    partners.update_status(1);
    match partners.get_status() {
        TeamOpenStatus::PlayerB => {}
        _ => {
            panic!(
                "Assertion failed, got {:?}, expected {:?}",
                partners.get_status(),
                TeamOpenStatus::PlayerB
            );
        }
    }

    partners.update_status(0);
    match partners.get_status() {
        TeamOpenStatus::Both => {}
        _ => {
            panic!(
                "Assertion failed, got {:?}, expected {:?}",
                partners.get_status(),
                TeamOpenStatus::Both
            );
        }
    }
}
