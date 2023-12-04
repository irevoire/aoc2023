use std::collections::HashMap;

use aoc::*;
use day4::*;

fn main() {
    let mut copy_by_id = HashMap::new();

    for card in parser::lines::<Card>() {
        let matching = card.matching_numbers();
        let copy = copy_by_id.entry(card.id).or_default();
        *copy += 1;
        let copy = *copy;

        for id in card.id + 1..=(card.id + matching) {
            *copy_by_id.entry(id).or_default() += copy;
        }
    }

    answer!(
        "You end up with {} scratchcards",
        copy_by_id.values().sum::<usize>()
    );
}
