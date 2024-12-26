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

use std::usize;

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

    println!("入れ替えたいカードの番号を入力してください（例： 1 2 3）");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    let numbers: Vec<usize> = input
        .split_whitespace() // 空白で分割
        .map(|x| x.parse().unwrap()) // 数値に変換
        .collect::<Vec<usize>>(); // Vec に変換
    for number in numbers {
        hand[number - 1] = deck.pop().unwrap();
    }

    // 手札をソート
    hand.sort_by(|a, b| a.rank.cmp(&b.rank));

    // 手札表示
    println!("----Hand----");
    for card in &hand {
        println!("{:?}: {:}", card.suit, card.rank);
    }
}
