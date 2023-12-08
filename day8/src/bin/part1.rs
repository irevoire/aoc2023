use aoc::*;

fn main() {
    let mut tree: Graph<&str, Direction, Directed> = Graph::new_directed();

    let input = parser::input::<String>();
    let (directions, nodes) = input.split_once("\n\n").unwrap();

    let directions = directions
        .trim()
        .chars()
        .map(|c| c.to_string().parse().unwrap())
        .collect::<Vec<Direction>>();

    for node in nodes.lines() {
        // AAA = (BBB, CCC)
        let (current, children) = node.split_once('=').unwrap();
        let current = current.trim();
        let (left, right) = children.split_once(',').unwrap();
        let [current, left, right] =
            [current, left, right].map(|s| s.trim_matches(|c: char| !c.is_alphabetic()));

        let current = tree.insert_value(current);
        let left = tree.insert_value(left);
        let right = tree.insert_value(right);

        if current != left {
            tree.create_edge_with_data(current, left, Direction::Left);
        }
        if current != right {
            tree.create_edge_with_data(current, right, Direction::Right);
        }
    }

    let mut steps = 0;
    let mut current = tree.get_id(&"AAA").unwrap();

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
