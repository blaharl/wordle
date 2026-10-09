use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Clone)]
pub struct Words {
    words: Vec<String>,
    hash_set: HashSet<String>,
}

impl Words {
    pub fn new() -> Self {
        let path = std::path::Path::new("words.txt");
        let words: Vec<String> = std::fs::read_to_string(path)
            .unwrap()
            .split('\n')
            .filter(|w| w.chars().all(|c| c.is_alphabetic()) && w.len() == 5)
            .map(|w| w.to_string().to_uppercase())
            .collect();

        let hash_set: HashSet<String> = words.clone().into_iter().collect();

        Self { words, hash_set }
    }

    fn generate_word(&self) -> String {
        let len = self.words.len();
        let index = rand::random_range(0..len);
        self.words[index].clone()
    }

    fn contains(&self, word: &String) -> bool {
        self.hash_set.contains(word)
    }

    pub fn generate_answer(&self) -> Answer {
        Answer::new(self.generate_word())
    }

    pub fn solve(
        &self,
        answer: &mut Answer,
        guess: &str,
    ) -> Result<(GuessState, Vec<Color>), InputError> {
        if answer.tries == 0 {
            return Err(InputError::NoMoreGuess);
        }

        if guess.len() != 5 || guess.chars().any(|c| !c.is_alphabetic()) {
            return Err(InputError::InvalidInput);
        }

        let guess = &guess.to_uppercase();

        if !self.contains(guess) {
            return Err(InputError::NotInList);
        }

        let guess = answer.guess(guess);

        if guess.iter().all(|&c| c == Color::Green) {
            answer.tries = 0;
            Ok((GuessState::Solved, guess))
        } else {
            answer.tries -= 1;
            if answer.tries == 0 {
                return Ok((GuessState::GameOver, guess));
            }
            Ok((GuessState::Wrong, guess))
        }
    }

    pub fn show(&self, answer: &Answer) -> (String, u8) {
        answer.show()
    }
}

impl Default for Words {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum Color {
    Green,
    Yellow,
    Gray,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum GuessState {
    Solved,
    Wrong,
    GameOver,
    Error(InputError),
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum InputError {
    InvalidInput,
    NotInList,
    NoMoreGuess,
}

impl std::fmt::Display for InputError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_str().to_owned())
    }
}

impl InputError {
    fn to_str(&self) -> String {
        match self {
            InputError::NotInList => "Input word is not in list".to_string(),
            InputError::InvalidInput => "Invalid input".to_string(),
            InputError::NoMoreGuess => "No more guesses allowed".to_string(),
        }
    }
}

pub struct Answer {
    word: String,
    tries: u8,
}

impl Answer {
    fn new(word: String) -> Self {
        Self { word, tries: 6 }
    }

    fn show(&self) -> (String, u8) {
        (self.word.clone(), self.tries)
    }

    fn guess(&self, guess: &str) -> Vec<Color> {
        let mut used = [false; 5];
        let mut result = vec![Color::Gray; 5];

        let answer = self.word.as_bytes();
        let guess = guess.as_bytes();

        for i in 0..5 {
            if answer[i] == guess[i] {
                result[i] = Color::Green;
                used[i] = true;
            }
        }

        for i in 0..5 {
            if result[i] != Color::Gray {
                continue;
            }

            for j in 0..5 {
                if used[j] {
                    continue;
                }

                if guess[i] == answer[j] {
                    result[i] = Color::Yellow;
                    used[j] = true;
                }
            }
        }

        result
    }
}
