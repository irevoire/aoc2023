use aoc::*;
use day5::*;

fn main() {
    let input = parser::input::<String>();
    let mut input = input.split("\n\n");
    let seeds: Seeds = input.next().unwrap().parse().unwrap();

    let mappers: Vec<Mapper> = input.map(|mapper| mapper.parse().unwrap()).collect();

    let ret = seeds
        .0
        .par_chunks(2)
        .flat_map(|seeds| {
            let mut locations = Vec::new();
            for seed in seeds[0]..(seeds[0] + seeds[1]) {
                let mut current = seed;
                for mapper in mappers.iter() {
                    current = mapper.map(current);
                }
                locations.push(current);
            }
            locations
        })
        .min()
        .unwrap();

    answer!("{}", ret);
}
