use itertools::Itertools;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use rand::{seq::SliceRandom, thread_rng};
use std::fmt;
use std::fmt::Display;
use strum::{EnumIter, IntoEnumIterator};

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
