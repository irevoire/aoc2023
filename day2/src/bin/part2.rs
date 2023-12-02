use aoc::*;
use day2::*;

fn main() {
    let ret: usize = parser::lines()
        .map(|game: Game| {
            game.sets
                .iter()
                .fold(Set::default(), |set, new| set.min(new))
        })
        .map(Set::power)
        .sum();

    answer!("The sum of the IDs of the possible games is {}", ret);
}
