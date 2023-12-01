use aoc::*;

fn main() {
    let output: usize = parser::lines::<String>()
        .map(|line| {
            let line = line
                .replace("one", "o1e")
                .replace("two", "t2o")
                .replace("three", "t3e")
                .replace("four", "f4r")
                .replace("five", "f5e")
                .replace("six", "s6x")
                .replace("seven", "s7n")
                .replace("eight", "e8t")
                .replace("nine", "n9e");
            // dbg!(&line);
            let mut iter = line
                .chars()
                .filter(char::is_ascii_digit)
                .map(|c| (c as u8 - b'0') as usize);

            let first = iter.next().unwrap();
            let last = iter.last().unwrap_or(first);

            first * 10 + last
        })
        // .map(|n| dbg!(n))
        .sum();

    answer!("The sum of all of the calibration values is {}.", output);
}
