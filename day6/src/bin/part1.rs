use aoc::{answer, parser};
use std::{str::FromStr, time::Duration};

struct Races(pub Vec<(Duration, usize)>);

impl FromStr for Races {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let line = s.lines().collect::<Vec<_>>();
        let times = line[0]
            .strip_prefix("Time:")
            .unwrap()
            .split_whitespace()
            .map(|dur| Duration::from_millis(dur.parse().unwrap()));
        let distances = line[1]
            .strip_prefix("Distance:")
            .unwrap()
            .split_whitespace()
            .map(|dur| dur.parse().unwrap());

        Ok(Races(times.into_iter().zip(distances).collect()))
    }
}

fn main() {
    let races = parser::input::<Races>();

    let mut ret = 1;

    for (duration, record) in races.0 {
        let duration = duration.as_millis() as usize;
        let mut count = 0;
        for i in 1..duration {
            if final_distance(i, duration) > record {
                count += 1;
            }
        }
        ret *= count;
    }

    answer!("{}", ret);
}

fn final_distance(start_after: usize, ends_at: usize) -> usize {
    (ends_at.saturating_sub(start_after)) * start_after
}
