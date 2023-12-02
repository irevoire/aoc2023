use aoc::*;
use day2::*;

fn main() {
    let (red, green, blue) = (12, 13, 14);

    let ret: usize = parser::lines()
        .filter(|game: &Game| {
            game.sets
                .iter()
                .all(|set| set.red <= red && set.green <= green && set.blue <= blue)
        })
        .map(|game| game.id)
        .sum();

    answer!("The sum of the IDs of the possible games is {}", ret);
}
