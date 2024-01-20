use crate::cards::{Card, Rank};
use itertools::Itertools;
use std::cmp::Ordering;
use std::fmt;
use std::fmt::Display;

#[derive(Clone, Debug)]
pub struct Hand([Card; 5]);

#[derive(Clone, PartialOrd, Eq, Ord, PartialEq, Debug)]
pub enum HandType {
    HighCard,
    Pair(Rank),
    TwoPair(Rank, Rank),
    ThreeOfAKind(Rank),
    Straight(Rank),
    Flush,
    FullHouse(Rank, Rank),
    FourOfAKind(Rank),
    StraightFlush(Rank),
}

impl Hand {
    pub fn from(cards: [Card; 5]) -> Self {
        let mut cards = cards.to_vec();
        cards.sort_by(|a, b| b.cmp(a));
        Hand(cards.try_into().expect("size should be 5"))
    }

    pub fn cards(&self) -> &[Card] {
        &self.0
    }

    pub fn best_hand(cards: &[Card]) -> Option<Self> {
        cards
            .iter()
            .cloned()
            .permutations(5)
            .map(|mut perm| {
                perm.sort_by(|a, b| b.cmp(a));
                Hand(perm.try_into().expect("size should be 5"))
            })
            .max()
    }

    pub fn hand_type(&self) -> HandType {
        if let Some(straight) = self.extract_straight() {
            if self.is_flush() {
                HandType::StraightFlush(straight)
            } else {
                HandType::Straight(straight)
            }
        } else if self.is_flush() {
            HandType::Flush
        } else {
            let ranks = self.ranks();
            let mut groups = ranks.into_iter().dedup_with_count().collect_vec();
            groups.sort_by(|a, b| b.cmp(a));
            match groups[0].0 {
                2 if groups[1].0 == 2 => HandType::TwoPair(groups[0].1, groups[1].1),
                2 => HandType::Pair(groups[0].1),
                3 if groups[1].0 == 2 => HandType::FullHouse(groups[0].1, groups[1].1),
                3 => HandType::ThreeOfAKind(groups[0].1),
                4 => HandType::FourOfAKind(groups[0].1),
                _ => HandType::HighCard,
            }
        }
    }

    fn is_flush(&self) -> bool {
        let suit = self.0[0].suit;
        for card in &self.0 {
            if suit != card.suit {
                return false;
            }
        }
        true
    }

    #[cfg(test)]
    fn is_straight(&self) -> bool {
        self.extract_straight().is_some()
    }

    fn ranks(&self) -> [Rank; 5] {
        self.0.map(|card| card.rank)
    }

    fn extract_straight(&self) -> Option<Rank> {
        let ranks = self.ranks();
        if let [Rank::Ace, Rank::Five, Rank::Four, Rank::Three, Rank::Two] = ranks {
            return Some(Rank::Five);
        }

        if ranks
            .iter()
            .zip(ranks.iter().dropping(1))
            .all(|(this, next)| *next != Rank::Ace && next.successor() == *this)
        {
            Some(ranks[0])
        } else {
            None
        }
    }
}

impl Eq for Hand {}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.hand_type().cmp(&other.hand_type()) {
            Ordering::Equal => self.ranks().cmp(&other.ranks()),
            other => other,
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Display for Hand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut output = "".to_string();
        let mut comma = false;
        for card in &self.0 {
            if comma {
                output.push_str(", ");
            }
            comma = true;
            output.push_str(&format!("{card}"));
        }
        write!(f, "{output}")
    }
}

#[cfg(test)]
mod tests {
    use crate::cards::*;
    use crate::hands::*;

    #[test]
    fn groups() {
        let hand = Hand::from([
            Card {
                rank: Rank::Two,
                suit: Suit::Diamonds,
            },
            Card {
                rank: Rank::Four,
                suit: Suit::Diamonds,
            },
            Card {
                rank: Rank::Four,
                suit: Suit::Clubs,
            },
            Card {
                rank: Rank::Five,
                suit: Suit::Diamonds,
            },
            Card {
                rank: Rank::Six,
                suit: Suit::Diamonds,
            },
        ]);
        assert_eq!(hand.hand_type(), HandType::Pair(Rank::Four));

        let hand = Hand::from([
            Card {
                rank: Rank::Two,
                suit: Suit::Diamonds,
            },
            Card {
                rank: Rank::Four,
                suit: Suit::Diamonds,
            },
            Card {
                rank: Rank::Four,
                suit: Suit::Clubs,
            },
            Card {
                rank: Rank::Five,
                suit: Suit::Diamonds,
            },
            Card {
                rank: Rank::Five,
                suit: Suit::Spades,
            },
        ]);
        assert_eq!(hand.hand_type(), HandType::TwoPair(Rank::Five, Rank::Four));

        let hand = Hand::from([
            Card {
                rank: Rank::Four,
                suit: Suit::Hearts,
            },
            Card {
                rank: Rank::Four,
                suit: Suit::Diamonds,
            },
            Card {
                rank: Rank::Four,
                suit: Suit::Clubs,
            },
            Card {
                rank: Rank::Five,
                suit: Suit::Diamonds,
            },
            Card {
                rank: Rank::Five,
                suit: Suit::Spades,
            },
        ]);
        assert_eq!(
            hand.hand_type(),
            HandType::FullHouse(Rank::Four, Rank::Five)
        );

        let hand = Hand::from([
            Card {
                rank: Rank::Four,
                suit: Suit::Hearts,
            },
            Card {
                rank: Rank::Four,
                suit: Suit::Diamonds,
            },
            Card {
                rank: Rank::Four,
                suit: Suit::Clubs,
            },
            Card {
                rank: Rank::Five,
                suit: Suit::Diamonds,
            },
            Card {
                rank: Rank::Ace,
                suit: Suit::Spades,
            },
        ]);
        assert_eq!(hand.hand_type(), HandType::ThreeOfAKind(Rank::Four));
    }

    #[test]
    fn straight() {
        let hand = Hand::from([
            Card {
                rank: Rank::Two,
                suit: Suit::Diamonds,
            },
            Card {
                rank: Rank::Four,
                suit: Suit::Diamonds,
            },
            Card {
                rank: Rank::Three,
                suit: Suit::Diamonds,
            },
            Card {
                rank: Rank::Five,
                suit: Suit::Diamonds,
            },
            Card {
                rank: Rank::Six,
                suit: Suit::Diamonds,
            },
        ]);
        assert_eq!(hand.hand_type(), HandType::StraightFlush(Rank::Six));

        let hand = Hand::from([
            Card {
                rank: Rank::Two,
                suit: Suit::Diamonds,
            },
            Card {
                rank: Rank::Four,
                suit: Suit::Diamonds,
            },
            Card {
                rank: Rank::Three,
                suit: Suit::Diamonds,
            },
            Card {
                rank: Rank::Five,
                suit: Suit::Diamonds,
            },
            Card {
                rank: Rank::Ace,
                suit: Suit::Diamonds,
            },
        ]);
        assert!(hand.is_straight());

        let hand = Hand::from([
            Card {
                rank: Rank::Two,
                suit: Suit::Diamonds,
            },
            Card {
                rank: Rank::Four,
                suit: Suit::Diamonds,
            },
            Card {
                rank: Rank::Three,
                suit: Suit::Diamonds,
            },
            Card {
                rank: Rank::King,
                suit: Suit::Diamonds,
            },
            Card {
                rank: Rank::Ace,
                suit: Suit::Diamonds,
            },
        ]);
        assert!(!hand.is_straight());

        let hand = Hand::from([
            Card {
                rank: Rank::Two,
                suit: Suit::Diamonds,
            },
            Card {
                rank: Rank::Eight,
                suit: Suit::Diamonds,
            },
            Card {
                rank: Rank::Three,
                suit: Suit::Diamonds,
            },
            Card {
                rank: Rank::King,
                suit: Suit::Diamonds,
            },
            Card {
                rank: Rank::Ace,
                suit: Suit::Diamonds,
            },
        ]);
        assert!(!hand.is_straight());

        let hand = Hand::from([
            Card {
                rank: Rank::Two,
                suit: Suit::Diamonds,
            },
            Card {
                rank: Rank::Four,
                suit: Suit::Diamonds,
            },
            Card {
                rank: Rank::Three,
                suit: Suit::Diamonds,
            },
            Card {
                rank: Rank::Ace,
                suit: Suit::Diamonds,
            },
            Card {
                rank: Rank::Ace,
                suit: Suit::Diamonds,
            },
        ]);
        assert!(!hand.is_straight());

        let hand = Hand::from([
            Card {
                rank: Rank::Ace,
                suit: Suit::Diamonds,
            },
            Card {
                rank: Rank::Queen,
                suit: Suit::Diamonds,
            },
            Card {
                rank: Rank::Jack,
                suit: Suit::Diamonds,
            },
            Card {
                rank: Rank::King,
                suit: Suit::Diamonds,
            },
            Card {
                rank: Rank::Ten,
                suit: Suit::Diamonds,
            },
        ]);
        assert!(hand.is_straight());
    }

    #[test]
    fn flush() {
        let hand = Hand::from([
            Card {
                rank: Rank::Two,
                suit: Suit::Diamonds,
            },
            Card {
                rank: Rank::Four,
                suit: Suit::Diamonds,
            },
            Card {
                rank: Rank::Three,
                suit: Suit::Diamonds,
            },
            Card {
                rank: Rank::Five,
                suit: Suit::Diamonds,
            },
            Card {
                rank: Rank::Six,
                suit: Suit::Diamonds,
            },
        ]);
        assert!(hand.is_flush());

        let hand = Hand::from([
            Card {
                rank: Rank::Two,
                suit: Suit::Diamonds,
            },
            Card {
                rank: Rank::Four,
                suit: Suit::Diamonds,
            },
            Card {
                rank: Rank::Three,
                suit: Suit::Diamonds,
            },
            Card {
                rank: Rank::Five,
                suit: Suit::Diamonds,
            },
            Card {
                rank: Rank::Six,
                suit: Suit::Clubs,
            },
        ]);
        assert!(!hand.is_flush());
    }
}
