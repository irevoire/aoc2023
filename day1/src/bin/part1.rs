use aoc::*;

fn main() {
    let output: usize = parser::lines::<String>()
        .map(|line| {
            let mut iter = line
                .chars()
                .filter(char::is_ascii_digit)
                .map(|c| (c as u8 - b'0') as usize);

            let first = iter.next().unwrap();
            let last = iter.last().unwrap_or(first);

            first * 10 + last
        })
        .sum();

    answer!("The sum of all of the calibration values is {}.", output);
}
