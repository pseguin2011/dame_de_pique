use card_game_engine::error::{CardGameError, DefaultCardGameError};
use std::{error as e, fmt};

#[derive(Debug)]
pub enum DameDePiqueError {
    IncorrectCardNumberRequest,
    DeckEmpty,
    InvalidOpeningHand(usize),
    InvalidDiscardOpeningHand(usize),
    PlayerCantAddPoints(usize),
    InvalidPoints,
}

impl fmt::Display for DameDePiqueError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DameDePiqueError::IncorrectCardNumberRequest => write!(f, "Too Many Cards were requested"),
            DameDePiqueError::DeckEmpty => write!(f, "The deck is empty"),
            DameDePiqueError::InvalidDiscardOpeningHand(player_id) => write!(f, "Player {} cannot pickup the deck", player_id),
            DameDePiqueError::InvalidOpeningHand(player_id) => write!(f, "Player {} cannot open", player_id),
            DameDePiqueError::PlayerCantAddPoints(player_id) => write!(f, "Player {} does not meet the requirements to add points", player_id),
            DameDePiqueError::InvalidPoints => write!(f, "The cards provided are not valid points for the game"),
        }
    }
}

impl e::Error for DameDePiqueError {
    fn description(&self) -> &str {
        match *self {
            DameDePiqueError::IncorrectCardNumberRequest =>  "Too Many Cards were requested",
            DameDePiqueError::DeckEmpty => "The deck is empty",
            DameDePiqueError::InvalidDiscardOpeningHand(_) => "Player could not pick up the discard pile",
            DameDePiqueError::InvalidOpeningHand(_) => "Player could not open",
            DameDePiqueError::InvalidPoints => "The cards provided are not valid points for the game",
            DameDePiqueError::PlayerCantAddPoints(_) => "Player could not add points",
        }
    }
}


impl CardGameError for DameDePiqueError {}

impl From<DefaultCardGameError> for DameDePiqueError {
    fn from(e: DefaultCardGameError) -> Self {
        match e {
            DefaultCardGameError::IncorrectCardNumberRequest =>  DameDePiqueError::IncorrectCardNumberRequest,
            DefaultCardGameError::DeckEmpty => DameDePiqueError::DeckEmpty,
        }
    }
}