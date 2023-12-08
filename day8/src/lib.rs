use aoc::*;

pub fn parse() -> (Vec<Direction>, Graph<String, Direction, Directed>) {
    let input = parser::input::<String>();
    let (directions, nodes) = input.split_once("\n\n").unwrap();

    let directions = directions
        .trim()
        .chars()
        .map(|c| c.to_string().parse().unwrap())
        .collect::<Vec<Direction>>();

    let mut tree: Graph<String, Direction, Directed> = Graph::new_directed();

    for node in nodes.lines() {
        // AAA = (BBB, CCC)
        let (current, children) = node.split_once('=').unwrap();
        let current = current.trim();
        let (left, right) = children.split_once(',').unwrap();
        let [current, left, right] =
            [current, left, right].map(|s| s.trim_matches(|c: char| !c.is_alphabetic()));

        let current = tree.insert_value(current.to_string());
        let left = tree.insert_value(left.to_string());
        let right = tree.insert_value(right.to_string());

        if current != left {
            tree.create_edge_with_data(current, left, Direction::Left);
        }
        if current != right {
            tree.create_edge_with_data(current, right, Direction::Right);
        }
    }

    (directions, tree)
}
