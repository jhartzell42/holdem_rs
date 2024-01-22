use crate::cards::Card;
use crate::hands::Hand;
use itertools::Itertools;

// Given community cards, find best two cards not in hand
// to win.
pub fn find_nuts(community: &[Card]) -> (Hand, [Card; 2]) {
    if community.len() < 3 {
        panic!("must provide at least 3 community cards");
    }

    Card::iter()
        .filter(|card| !community.contains(&card))
        .combinations(2)
        .map(|cards| {
            let mut total_cards = community.to_vec();
            total_cards.extend_from_slice(&cards);
            (
                Hand::best_hand(&total_cards),
                cards.try_into().expect("correct size"),
            )
        })
        .max()
        .expect("there are many combinations of at least 2 cards")
}
