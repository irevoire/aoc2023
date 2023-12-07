use std::collections::BinaryHeap;

use aoc::{answer, parser};
use std::{cmp::Ordering, collections::HashMap, str::FromStr};

#[derive(Debug, PartialEq, Eq)]
pub struct Player {
    pub hand: Hand,
    pub bid: usize,
}

impl Ord for Player {
    fn cmp(&self, other: &Self) -> Ordering {
        self.hand.cmp(&other.hand)
    }
}
impl PartialOrd for Player {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl FromStr for Player {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (hand, bid) = s.split_once(' ').unwrap();

        Ok(Player {
            hand: hand.parse()?,
            bid: bid.parse().unwrap(),
        })
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Hand {
    // a representation of your hand
    hand: Type,
    // the raw unaltered cards
    cards: Vec<Card>,
}

impl FromStr for Hand {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cards = s
            .chars()
            .map(|c| match c {
                'A' => Card::Head(Head::As),
                'K' => Card::Head(Head::King),
                'Q' => Card::Head(Head::Queen),
                'J' => Card::Head(Head::Jack),
                'T' => Card::Head(Head::Truc),
                n => Card::Number(n.to_digit(10).expect(&format!("wtf is {n}")) as u8),
            })
            .collect::<Vec<_>>();

        let mut hand = HashMap::new();
        for card in cards.iter() {
            *hand.entry(*card).or_default() += 1;
        }
        let mut hand: Vec<usize> = hand.values().copied().collect();
        hand.sort_by(|left, right| left.cmp(right).reverse());
        let hand = match hand.as_slice() {
            [5] => Type::FiveOfAKind,
            [4, 1] => Type::FourOfAKind,
            [3, 2] => Type::FullHouse,
            [3, 1, 1] => Type::ThreeOfAKind,
            [2, 2, 1] => Type::TwoPair,
            [2, 1, 1, 1] => Type::OnePair,
            [1, 1, 1, 1, 1] => Type::HighCard,
            hand => panic!("Don't understand the type of {hand:?}"),
        };

        Ok(Hand { hand, cards })
    }
}

#[derive(Copy, Clone, Hash, Debug, PartialEq, Eq)]
pub enum Card {
    Head(Head),
    Number(u8),
}

#[derive(Copy, Clone, Hash, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Head {
    Truc = 0,
    Jack = 1,
    Queen = 2,
    King = 3,
    As = 4,
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> Ordering {
        use Card::*;

        match (self, other) {
            (Head(left), Head(right)) => left.cmp(right),
            (Head(_), Number(_)) => Ordering::Greater,
            (Number(_), Head(_)) => Ordering::Less,
            (Number(left), Number(right)) => left.cmp(right),
        }
    }
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
pub enum Type {
    HighCard = 0,     // where all cards' labels are distinct: 23456
    OnePair = 1, // where two cards share one label, and the other three cards have a different label from the pair and each other: A23A4
    TwoPair = 2, // where two cards share one label, two other cards share a second label, and the remaining card has a third label: 23432
    ThreeOfAKind = 3, // where three cards have the same label, and the remaining two cards are each different from any other card in the hand: TTT98
    FullHouse = 4, // where three cards have the same label, and the remaining two cards share a different label: 23332
    FourOfAKind = 5, // where four cards have the same label and one card has a different label: AA8AA
    FiveOfAKind = 6, // where all five cards have the same label: AAAAA
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        self.hand
            .cmp(&other.hand)
            .then(self.cards[0].cmp(&other.cards[0]))
            .then(self.cards[1].cmp(&other.cards[1]))
            .then(self.cards[2].cmp(&other.cards[2]))
            .then(self.cards[3].cmp(&other.cards[3]))
            .then(self.cards[4].cmp(&other.cards[4]))
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    let ret: usize = parser::lines::<Player>()
        .collect::<BinaryHeap<Player>>()
        .into_sorted_vec()
        .into_iter()
        .enumerate()
        .map(|(rank, player)| (rank + 1) * player.bid)
        .sum();

    answer!("The total winnings are {}", ret);
}
