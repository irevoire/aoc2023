use aoc::*;
use day6::*;

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
