use game::gameplay::DDPState;
use game::models::CardValue;
use serde::{Deserialize, Serialize};

use std::collections::HashMap;

#[derive(Clone, Debug, Deserialize)]
pub struct GameDiscardRequest {
    pub game_id: String,
    pub card_index: usize,
}

#[derive(Clone, Debug, Deserialize)]
pub struct PlayerPickupDiscardRequest {
    pub game_id: String,
    pub card_indices: Vec<usize>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct PlayerOpenRequest {
    pub game_id: String,
    pub card_indices: Vec<usize>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct PlayerAddPointsRequest {
    pub game_id: String,
    pub card_indices: Vec<usize>,
}

#[derive(Clone, Debug, Serialize)]
pub struct PlayerGameStateResponse {
    pub player_hand: Vec<Card>,
    team1_points: HashMap<String, Vec<Card>>,
    team2_points: HashMap<String, Vec<Card>>,
    team_1_total_points: i16,
    team_2_total_points: i16,
    top_discard: Option<Card>,
    turn: usize,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Card {
    suit: String,
    value: String,
}

impl From<DDPState> for PlayerGameStateResponse {
    fn from(other: DDPState) -> PlayerGameStateResponse {
        let team1_points: HashMap<String, Vec<Card>> = other.partners[0]
            .points_deck
            .clone()
            .into_iter()
            .map(|(k, v)| {
                (
                    format!("{:?}", k),
                    v.iter().map(|bb| Card::from(bb.clone())).collect(),
                )
            })
            .collect();

        let team2_points: HashMap<String, Vec<Card>> = other.partners[1]
            .points_deck
            .clone()
            .into_iter()
            .map(|(k, v)| {
                (
                    format!("{:?}", k),
                    v.iter().map(|bb| Card::from(bb.clone())).collect(),
                )
            })
            .collect();

        let team_1_total_points = other.partners[0].total_points;
        let team_2_total_points = other.partners[1].total_points;

        let top_discard = match other.default_state.deck.peek_top_discarded_card() {
            Some(card) => Some(Card::from(card.clone())),
            None => None,
        };
        PlayerGameStateResponse {
            player_hand: vec![],
            team1_points,
            team2_points,
            team_1_total_points,
            team_2_total_points,
            turn: other.default_state.turn,
            top_discard,
        }
    }
}

impl From<game::models::Card> for Card {
    fn from(other: game::models::Card) -> Card {
        let value = match other.value {
            CardValue::Ace => "A".into(),
            CardValue::Two => "2".into(),
            CardValue::Three => "3".into(),
            CardValue::Four => "4".into(),
            CardValue::Five => "5".into(),
            CardValue::Six => "6".into(),
            CardValue::Seven => "7".into(),
            CardValue::Eight => "8".into(),
            CardValue::Nine => "9".into(),
            CardValue::Ten => "10".into(),
            CardValue::Jack => "J".into(),
            CardValue::Queen => "Q".into(),
            CardValue::King => "K".into(),
            CardValue::Joker => "Joker".into(),
        };
        Card {
            suit: format!("{:?}", other.suit),
            value,
        }
    }
}
