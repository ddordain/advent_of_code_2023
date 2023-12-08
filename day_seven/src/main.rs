use std::{collections::HashMap};

#[derive(Debug)]
enum Combo {
    HighCard = 1,
    Pair = 10,
    DoublePair = 100,
    Triple = 1_000,
    Full = 10_000,
    Four = 100_000,
    Five = 1_000_000,
}

#[derive(Debug)]
enum Card {
    Two = 2,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    J,
    Q,
    K,
    A,
}

fn count_hand(hand : &str) -> HashMap<char, u8> {
    let mut count = HashMap::new();

    for card in hand.chars() {
        count.entry(card).and_modify(|counter| *counter+=1).or_insert(1);
    }
    count
}

fn rank_hand(hand: HashMap<char, u8>) -> Combo {
    let mut counts: Vec<u8> = hand.values().cloned().collect();
    counts.sort();

    match counts.as_slice() {
        [5] => Combo::Five,
        [1, 4] => Combo::Four,
        [2, 3] => Combo::Full,
        [1, 1, 3] => Combo::Triple,
        [1, 2, 2]  => Combo::DoublePair,
        [1, 1, 1, 2] => Combo::Pair,
        [1, 1, 1, 1, 1] => Combo::HighCard,
        _ => unreachable!(), 
    }
}

fn main() {
    println!("{:?}", Card::A as u8);
    let cards = "T8VVA";
    let rank = count_hand(cards);
    println!("{:?}", rank);
    let rank = rank_hand(rank);
    println!("{:?}", rank as u64);
}
