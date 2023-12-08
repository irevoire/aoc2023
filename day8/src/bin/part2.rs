use aoc::*;

fn main() {
    let (directions, tree) = day8::parse();

    let (starts, ends): (Vec<_>, Vec<_>) = tree
        .values()
        .filter(|s| s.ends_with(|c| c == 'A' || c == 'Z'))
        .partition(|s| s.ends_with('A'));

    let mut currents: Vec<_> = starts
        .into_iter()
        .map(|s| tree.get_id(s).unwrap())
        .collect();
    let ends: Vec<_> = ends.into_iter().map(|s| tree.get_id(s).unwrap()).collect();

    let mut steps = 0;
    for direction in directions.iter().cycle() {
        if currents.iter().all(|current| ends.contains(current)) {
            break;
        }

        for current in currents.iter_mut() {
            let edges = tree.get_edges(*current);
            *current = edges.iter().find(|(_, dir)| dir == direction).unwrap().0;
        }
        steps += 1;
    }

    answer!("{} steps are required to reach ZZZ", steps);
}
