use aoc::*;
use day5::*;

fn main() {
    let input = parser::input::<String>();
    let mut input = input.split("\n\n");
    let seeds: Seeds = input.next().unwrap().parse().unwrap();

    let mappers: Vec<Mapper> = input.map(|mapper| mapper.parse().unwrap()).collect();

    let mut ret = u64::MAX;

    for seed in seeds.0 {
        let mut current = seed;
        for mapper in mappers.iter() {
            current = mapper.map(current);
        }
        ret = ret.min(current);
    }

    answer!("{}", ret);
}
