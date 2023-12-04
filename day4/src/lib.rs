use std::{collections::HashSet, str::FromStr};

#[derive(Debug)]
pub struct Card {
    pub id: usize,
    winning: HashSet<usize>,
    numbers: Vec<usize>,
}

impl Card {
    pub fn points(self) -> usize {
        self.numbers
            .iter()
            .filter(|n| self.winning.contains(n))
            .fold(0, |acc, _n| if acc == 0 { acc + 1 } else { acc * 2 })
    }

    pub fn matching_numbers(&self) -> usize {
        self.numbers
            .iter()
            .filter(|n| self.winning.contains(n))
            .count()
    }
}

impl FromStr for Card {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(|c| c == ':' || c == '|');
        let id = parts.next().unwrap().strip_prefix("Card ").unwrap();
        let winning = parts
            .next()
            .unwrap()
            .split_whitespace()
            .map(str::trim)
            .map(|n| n.parse().unwrap())
            .collect();

        let numbers = parts
            .next()
            .unwrap()
            .split_whitespace()
            .map(str::trim)
            .map(|n| n.parse().unwrap())
            .collect();

        Ok(Card {
            id: id.trim().parse().unwrap(),
            winning,
            numbers,
        })
    }
}
