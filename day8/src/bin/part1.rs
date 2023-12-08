use aoc::*;

fn main() {
    let (directions, tree) = day8::parse();

    let mut steps = 0;
    let mut current = tree.get_id(&"AAA".to_string()).unwrap();

    for direction in directions.iter().cycle() {
        if *tree.get_value(current).unwrap() == "ZZZ" {
            break;
        };
        let edges = tree.get_edges(current);
        current = edges.iter().find(|(_, dir)| dir == direction).unwrap().0;
        steps += 1;
    }

    answer!("{} steps are required to reach ZZZ", steps);
}
