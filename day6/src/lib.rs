use std::{str::FromStr, time::Duration};

pub struct Races(pub Vec<(Duration, usize)>);

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
