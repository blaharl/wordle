use crate::words::{Color, GuessState};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ServerMsg {
    guess_state: GuessState,
    tries: u8,
    colors: Vec<Color>,
}

impl ServerMsg {
    pub fn new(guess_state: GuessState, tries: u8, colors: Vec<Color>) -> Self {
        Self {
            guess_state,
            tries,
            colors,
        }
    }

    pub fn to_string(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }

    pub fn from_slice(slice: &[u8]) -> Self {
        serde_json::from_slice(slice).unwrap()
    }

    pub fn guess_state(&self) -> GuessState {
        self.guess_state
    }

    pub fn tries(&self) -> u8 {
        self.tries
    }

    pub fn colors(&self) -> Vec<Color> {
        self.colors.clone()
    }
}

#[derive(Serialize, Deserialize)]
pub struct ClientMsg {
    guess: String,
}

impl ClientMsg {
    pub fn new(guess: String) -> Self {
        Self { guess }
    }

    pub fn to_string(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }

    pub fn from_slice(slice: &[u8]) -> Self {
        serde_json::from_slice(slice).unwrap()
    }

    pub fn guess(&self) -> String {
        self.guess.clone()
    }
}
