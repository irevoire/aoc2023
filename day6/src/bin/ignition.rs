use aoc::{answer, parser};
use std::iter::zip;
use std::str::FromStr;

struct Races(pub (f32, f32));

impl FromStr for Races {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let line = s.lines().collect::<Vec<_>>();
        let times = line[0]
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
    answer!("{}", run2());
}

fn run2() -> u32 {
    let races = parser::input::<Races>();

    let (duration, record) = races.0;

    linear_bounds(duration, record)
}
fn linear_bounds(time: f32, distance: f32) -> u32 {
    // distance > |x| (time - x) * x;
    // (time - x) * x - distance > 0;
    // x * time - x*x - distance > 0;
    // -x2 + time * x - distance > 0;
    // a = -1, b = time, c = -distance;
    // delta = b2 - 4ac;
    // si delta positif 2 solutions;
    // si delta = 0 1 solution;
    // si delta netatif solution pas dans R;
    let delta = (time * time) - (4.0 * -1.0 * -(distance));
    if delta.is_sign_positive() {
        // -b - racine de delta / 2a
        // -b + racine de delta / 2a
        let one = (-time as f32 - f32::sqrt(delta)) / (2 * -1) as f32;
        let two = (-time as f32 + f32::sqrt(delta)) / (2 * -1) as f32;

        let first = f32::min(one, two);
        let second = f32::max(one, two);
        // first and second solve for = 0, if these values are round, we need to discard the exact bound
        let first = if first.fract().abs() < f32::EPSILON {
            first.round() as i32 + 1
        } else {
            first.ceil() as i32
        };
        let second = if second.fract().abs() < f32::EPSILON {
            second.round() as i32 - 1
        } else {
            second.floor() as i32
        };

        (second - first).abs() as u32 + 1
    } else if delta.abs() < f32::EPSILON {
        1
    } else {
        0
    }
}
