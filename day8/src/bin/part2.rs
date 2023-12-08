use aoc::*;

fn main() {
    let (directions, tree) = day8::parse();

    let (starts, ends): (Vec<_>, Vec<_>) = tree
        .values()
        .filter(|s| s.ends_with(|c| c == 'A' || c == 'Z'))
        .partition(|s| s.ends_with('A'));

    let currents: Vec<_> = starts
        .into_iter()
        .map(|s| tree.get_id(s).unwrap())
        .collect();
    let ends: Vec<_> = ends.into_iter().map(|s| tree.get_id(s).unwrap()).collect();

    let loops = currents
        .par_iter()
        .map(|current| {
            let mut current = *current;
            let mut ret = 0;

            for (step, direction) in directions.iter().cycle().enumerate() {
                if ends.contains(&current) {
                    ret = step;
                    break;
                }

                let edges = tree.get_edges(current);
                current = edges.iter().find(|(_, dir)| dir == direction).unwrap().0;
            }
            ret
        })
        .collect::<Vec<usize>>();

    let steps = loops.iter().copied().reduce(lcm).unwrap();

    answer!("{} steps are required to reach ZZZ", steps);
}

/// Least Common Multiple
/// Stolen from https://www.hackertouch.com/least-common-multiple-in-rust.html
fn lcm(first: usize, second: usize) -> usize {
    first * second / gcd(first, second)
}

/// Greatest Common Divisor
fn gcd(first: usize, second: usize) -> usize {
    let mut max = first;
    let mut min = second;
    if min > max {
        let val = max;
        max = min;
        min = val;
    }

    loop {
        let res = max % min;
        if res == 0 {
            return min;
        }

        max = min;
        min = res;
    }
}
