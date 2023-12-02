use std::str::FromStr;

#[derive(Debug)]
pub struct Game {
    pub id: usize,
    pub sets: Vec<Set>,
}

impl FromStr for Game {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (id, sets) = s.split_once(':').unwrap();
        let id = id.trim_start_matches("Game ").parse().unwrap();
        let sets = sets.split(';').map(|set| set.parse().unwrap()).collect();
        Ok(Game { id, sets })
    }
}

#[derive(Debug, Default)]
pub struct Set {
    pub red: usize,
    pub green: usize,
    pub blue: usize,
}

impl FromStr for Set {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut set = Set::default();
        s.split(',').map(str::trim).for_each(|cubes| {
            let cubes = cubes.split(' ').map(str::trim).collect::<Vec<_>>();
            match cubes[1] {
                "blue" => set.blue += cubes[0].parse::<usize>().unwrap(),
                "red" => set.red += cubes[0].parse::<usize>().unwrap(),
                "green" => set.green += cubes[0].parse::<usize>().unwrap(),
                s => unreachable!("{s}"),
            };
        });
        Ok(set)
    }
}
