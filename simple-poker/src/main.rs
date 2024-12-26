#[derive(Debug, Clone, Copy, PartialEq)]
enum Suit {
    Club,
    Diamond,
    Heart,
    Spade,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Card {
    suit: Suit,
    rank: i32,
}

use rand::prelude::SliceRandom;

fn main() {
    let mut deck: Vec<Card> = Vec::new();
    let suits = [Suit::Club, Suit::Diamond, Suit::Heart, Suit::Spade];

    // デッキ生成
    for suit in suits {
        for rank in 1..=13 {
            deck.push(Card { suit, rank });
        }
    }

    // デッキをシャッフル
    let mut rng = rand::thread_rng();
    deck.shuffle(&mut rng);

    // 手札
    let mut hand: Vec<Card> = Vec::new();

    for _ in 0..5 {
        hand.push(deck.pop().unwrap());
    }

    // 手札をソート
    hand.sort_by(|a, b| a.rank.cmp(&b.rank));

    // 手札表示
    println!("----Hand----");
    for card in &hand {
        println!("{:?}: {:}", card.suit, card.rank);
    }
}
