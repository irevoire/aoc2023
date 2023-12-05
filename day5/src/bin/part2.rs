use aoc::*;
use day5::*;

fn main() {
    let input = parser::input::<String>();
    let mut input = input.split("\n\n");
    let seeds: Seeds = input.next().unwrap().parse().unwrap();

    let mappers: Vec<Mapper> = input.map(|mapper| mapper.parse().unwrap()).collect();

    let mut ret = u64::MAX;

    for seeds in seeds.0.chunks(2) {
        let mut current = vec![seeds[0]..seeds[0] + seeds[1]];

        for mapper in mappers.iter() {
            let mut next = Vec::new();
            for range in current {
                next.append(&mut mapper.map_range(range));
            }
            current = next;
        }
        ret = ret.min(current.iter().map(|range| range.start).min().unwrap());
    }

    answer!("{}", ret);
}
