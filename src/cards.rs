use itertools::Itertools;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use rand::{seq::SliceRandom, thread_rng};
use std::fmt;
use std::fmt::Display;
use std::str::FromStr;
use strum::{EnumIter, IntoEnumIterator};
use thiserror::Error;

#[derive(Clone, Debug)]
pub struct Deck(pub Vec<Card>);

impl Deck {
    pub fn new() -> Self {
        let mut vec = Card::iter().collect_vec();
        vec.as_mut_slice().shuffle(&mut thread_rng());
        Deck(vec)
    }

    pub fn deal_n<const N: usize>(&mut self) -> Option<[Card; N]> {
        if N > self.0.len() {
            None
        } else {
            self.0.split_off(self.0.len() - N).try_into().ok()
        }
    }
}

#[derive(Clone, Debug, Copy, PartialEq, PartialOrd, Eq, Ord)]
pub struct Card {
    pub rank: Rank,
    pub suit: Suit,
}

impl Card {
    pub fn iter() -> impl Iterator<Item = Self> + Clone {
        Rank::iter()
            .cartesian_product(Suit::iter())
            .map(|(rank, suit)| Card { rank, suit })
    }
}

impl Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.rank, self.suit)
    }
}

#[derive(Error, Debug)]
pub enum CardParseError {
    #[error("error parsing rank: {0}")]
    RankParseError(#[from] RankParseError),

    #[error("error parsing suit: {0}")]
    SuitParseError(#[from] SuitParseError),

    #[error("insufficient length")]
    IncompleteError,
}

impl FromStr for Card {
    type Err = CardParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        if s.len() < 2 {
            return Err(CardParseError::IncompleteError);
        }

        let (rank_str, suit_str) = if &s[0..2] == "10" {
            s.split_at(2)
        } else {
            s.split_at(1)
        };

        let rank = rank_str.parse::<Rank>()?;
        let suit = suit_str.parse::<Suit>()?;

        Ok(Card { rank, suit })
    }
}

#[derive(Clone, Copy, PartialEq, PartialOrd, Debug, EnumIter, Eq, Ord, FromPrimitive)]
pub enum Rank {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl Rank {
    pub fn successor(&self) -> Self {
        FromPrimitive::from_u8(*self as u8 + 1).unwrap_or(Rank::Two)
    }
}

impl Display for Rank {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = match self {
            Rank::Two => "2",
            Rank::Three => "3",
            Rank::Four => "4",
            Rank::Five => "5",
            Rank::Six => "6",
            Rank::Seven => "7",
            Rank::Eight => "8",
            Rank::Nine => "9",
            Rank::Ten => "10",
            Rank::Jack => "J",
            Rank::Queen => "Q",
            Rank::King => "K",
            Rank::Ace => "A",
        };
        write!(f, "{name}")
    }
}

#[derive(Error, Debug)]
pub enum RankParseError {
    #[error("invalid rank: {0}")]
    InvalidRank(String),
}

impl FromStr for Rank {
    type Err = RankParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "2" => Ok(Rank::Two),
            "3" => Ok(Rank::Three),
            "4" => Ok(Rank::Four),
            "5" => Ok(Rank::Five),
            "6" => Ok(Rank::Six),
            "7" => Ok(Rank::Seven),
            "8" => Ok(Rank::Eight),
            "9" => Ok(Rank::Nine),
            "10" => Ok(Rank::Ten),
            "J" | "j" => Ok(Rank::Jack),
            "Q" | "q" => Ok(Rank::Queen),
            "K" | "k" => Ok(Rank::King),
            "A" | "a" => Ok(Rank::Ace),
            _ => Err(RankParseError::InvalidRank(s.to_string())),
        }
    }
}

#[derive(Clone, Copy, PartialEq, PartialOrd, Ord, Debug, EnumIter, Eq)]
pub enum Suit {
    Hearts,
    Clubs,
    Spades,
    Diamonds,
}

impl Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = match self {
            Suit::Hearts => "♥",
            Suit::Clubs => "♣",
            Suit::Spades => "♠",
            Suit::Diamonds => "♦",
        };
        write!(f, "{name}")
    }
}

#[derive(Error, Debug)]
pub enum SuitParseError {
    #[error("invalid suit: {0}")]
    InvalidSuit(String),
}

impl FromStr for Suit {
    type Err = SuitParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "♥" | "h" | "H" => Ok(Suit::Hearts),
            "♣" | "c" | "C" => Ok(Suit::Clubs),
            "♠" | "s" | "S" => Ok(Suit::Spades),
            "♦" | "d" | "D" => Ok(Suit::Diamonds),
            _ => Err(SuitParseError::InvalidSuit(s.to_string())),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::cards::*;

    #[test]
    fn parse_and_display() {
        let card = "10d".parse::<Card>().expect("should not fail");
        assert_eq!(format!("{card}"), "10♦");
        let card = "AC\n".parse::<Card>().expect("should not fail");
        assert_eq!(format!("{card}"), "A♣");
        let card = "ac".parse::<Card>().expect("should not fail");
        assert_eq!(format!("{card}"), "A♣");
    }
}
