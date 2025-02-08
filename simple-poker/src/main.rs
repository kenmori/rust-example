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

use rand::seq::SliceRandom;
fn main() {
    let mut deck: Vec<Card> = Vec::new();
    let suits = [Suit::Club, Suit::Diamond, Suit::Heart, Suit::Spade];

    for suit in suits {
        for rank in 1..=13 {
            deck.push(Card { suit, rank });
        }
    }

    let mut rng = rand::rng();
    deck.shuffle(&mut rng);

    let mut hand: Vec<Card> = Vec::new();

    for _ in 0..5 {
        hand.push(deck.pop().unwrap());
    }

    hand.sort_by(|a, b| a.rank.cmp(&b.rank));
    println!("--hand---");

    for (i, card) in hand.iter().enumerate() {
        println!("{}: {:?} {}", i + 1, card.suit, card.rank);
    }

    let suit = hand.first().unwrap().suit;
    let flash = hand.iter().all(|c| c.suit == suit);

    let mut count = 0;

    for i in 0..hand.len() - 1 {
        for j in i + 1..hand.len() {
            if hand[i].rank == hand[j].rank {
                count += 1;
            }
        }
    }

    if flash {
        println!("Flash!!");
    } else if count >= 3 {
        println!("Three card!!");
    } else if count == 2 {
        println!("2 pair!!");
    } else if count == 1 {
        println!("1 pair!!");
    } else {
        println!("No pair!!");
    }
}
