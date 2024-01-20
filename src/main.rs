mod cards;
mod hands;
mod hold_em;

fn main() {
    let mut deck = cards::Deck::new();
    let flop = deck.deal_n::<3>().expect("deck too small");
    println!("Flop: {} {} {}", flop[0], flop[1], flop[2]);
    let (hand, cards) = hold_em::find_nuts(&flop).expect("can always find nuts with flop of 3");
    println!("Nut cards: {} {}", cards[0], cards[1]);
    println!("Nut hand: {hand}");
    println!("This is a {:?}", hand.hand_type());
}
