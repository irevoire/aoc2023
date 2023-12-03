use std::{
    collections::{HashMap, HashSet},
    fmt::{self},
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
};

use aoc::*;

#[derive(Debug, Clone)]
enum Cell {
    Number(Arc<AtomicUsize>),
    Symbol(char),
    Nothing,
}
impl Cell {
    pub fn is_number(&self) -> bool {
        matches!(self, Cell::Number(_))
    }

    pub fn is_symbol(&self) -> bool {
        matches!(self, Cell::Symbol(_))
    }
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Cell::Number(n) => write!(f, "{:3?}", n),
            Cell::Symbol(c) => write!(f, " {c} "),
            Cell::Nothing => f.write_str(" . "),
        }
    }
}

fn main() {
    let grid = parser::lines()
        .map(|line: String| {
            line.chars()
                .map(|c| match c {
                    n @ '0'..='9' => {
                        let n = n.to_digit(10).unwrap() as usize;
                        let cell = Arc::new(AtomicUsize::new(n));

                        Cell::Number(cell)
                    }
                    '.' => Cell::Nothing,
                    c if c.is_ascii_punctuation() => Cell::Symbol(c),
                    c => panic!("What is {c}"),
                })
                .collect::<Vec<Cell>>()
        })
        .collect::<Vec<_>>();

    let mut grid: Grid<Cell> = Grid::from(grid);

    for coord in Coord::at(0, 0)
        .to(Coord::at(grid.width() - 1, grid.height() - 1))
        .unwrap()
    {
        if let Some(Cell::Number(prev)) = coord
            .checked_add(Direction::West)
            .and_then(|coord| grid.get(coord))
        {
            let prev = prev.clone();
            if let Some(Cell::Number(ref mut cell)) = grid.get_mut(coord) {
                let current = cell.load(Ordering::Relaxed);
                *cell = prev;
                cell.fetch_update(Ordering::Release, Ordering::Acquire, |old| {
                    Some(old * 10 + current)
                })
                .unwrap();
            }
        }
    }

    println!("{grid}");

    let mut part_number: HashMap<*mut usize, usize> = HashMap::new();

    for (coord, cell) in grid.enumerate() {
        if let Cell::Number(n) = cell {
            // check around
            for coord in [
                coord
                    .checked_add(Direction::North)
                    .and_then(|coord| coord.checked_add(Direction::West)),
                coord.checked_add(Direction::North),
                coord
                    .checked_add(Direction::North)
                    .map(|coord| coord + Direction::East),
                coord.checked_add(Direction::West),
                Some(coord + Direction::East),
                coord
                    .checked_add(Direction::West)
                    .map(|coord| coord + Direction::South),
                Some(coord + Direction::South),
                Some(coord + Direction::South + Direction::East),
            ]
            .iter()
            .filter_map(|coord| *coord)
            {
                if let Some(Cell::Symbol(_)) = grid.get(coord) {
                    part_number.insert(n.as_ptr(), n.load(Ordering::Relaxed));
                }
            }
        }
    }

    let ret: usize = part_number.values().sum();

    answer!("{}", ret);

    // part 2
    let mut gear_ratio = 0;

    for (coord, cell) in grid.enumerate() {
        if let Cell::Symbol('*') = cell {
            let mut numbers = Vec::new();
            // check around
            for coord in [
                coord
                    .checked_add(Direction::North)
                    .and_then(|coord| coord.checked_add(Direction::West)),
                coord.checked_add(Direction::North),
                coord
                    .checked_add(Direction::North)
                    .map(|coord| coord + Direction::East),
                coord.checked_add(Direction::West),
                Some(coord + Direction::East),
                coord
                    .checked_add(Direction::West)
                    .map(|coord| coord + Direction::South),
                Some(coord + Direction::South),
                Some(coord + Direction::South + Direction::East),
            ]
            .iter()
            .filter_map(|coord| *coord)
            {
                if let Some(Cell::Number(cell)) = grid.get(coord) {
                    if part_number.contains_key(&cell.as_ptr()) {
                        numbers.push(cell.as_ptr());
                    }
                }
            }
            numbers.sort_unstable();
            numbers.dedup();
            if numbers.len() == 2 {
                let a = part_number[&numbers[0]];
                let b = part_number[&numbers[1]];
                gear_ratio += a * b;
            }
        }
    }

    answer!("{}", gear_ratio);
}
