use aoc::{answer, parser};
use std::str::FromStr;

struct Races(pub (usize, usize));

impl FromStr for Races {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let line = s.lines().collect::<Vec<_>>();
        let times: usize = line[0]
            .strip_prefix("Time:")
            .unwrap()
            .replace(' ', "")
            .parse()
            .unwrap();
        let distances = line[1]
            .strip_prefix("Distance:")
            .unwrap()
            .replace(' ', "")
            .parse()
            .unwrap();

        Ok(Races((times, distances)))
    }
}

fn main() {
    let races = parser::input::<Races>();

    let (duration, record) = races.0;

    let mut min = 0;
    for i in 1..duration {
        if final_distance(i, duration) > record {
            min = i;
            break;
        }
    }

    let mut max = 0;
    for i in (1..duration).rev() {
        if final_distance(i, duration) > record {
            max = i;
            break;
        }
    }

    answer!("{}", max - min + 1);
}

fn final_distance(start_after: usize, ends_at: usize) -> usize {
    (ends_at.saturating_sub(start_after)) * start_after
}
