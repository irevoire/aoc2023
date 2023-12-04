use aoc::*;
use day4::*;

fn main() {
    let ret: usize = parser::lines::<Card>().map(Card::points).sum();

    answer!("{}", ret);
}
