use std::{ops::Range, str::FromStr};

#[derive(Debug)]
pub struct Seeds(pub Vec<u64>);

impl FromStr for Seeds {
    type Err = aoc::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Seeds(
            s.strip_prefix("seeds: ")
                .unwrap()
                .split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect(),
        ))
    }
}

#[derive(Debug)]
pub struct Mapper {
    _name: String,

    ranges: Vec<Map>,
}

impl Mapper {
    pub fn map(&self, value: u64) -> u64 {
        for range in self.ranges.iter() {
            if let Some(value) = range.try_map(value) {
                return value;
            }
        }
        value
    }

    pub fn map_range(&self, range: Range<u64>) -> Vec<Range<u64>> {
        let mut ranges = Vec::new();

        for map in self.ranges.iter() {
            if let Some(range) = map.try_map_range(&range) {
                ranges.push(range);
            }
        }

        ranges
    }
}

impl FromStr for Mapper {
    type Err = ();

    // seed-to-soil map:
    // 50 98 2
    // 52 50 48
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let name = lines.next().unwrap().strip_suffix(" map:").unwrap();
        let ranges = lines.map(|line| line.parse().unwrap()).collect();

        Ok(Mapper {
            _name: name.to_string(),
            ranges,
        })
    }
}

#[derive(Debug)]
pub struct Map {
    src: u64,
    dest: u64,
    size: u64,
}

impl Map {
    pub fn try_map(&self, value: u64) -> Option<u64> {
        (self.src..self.src + self.size)
            .contains(&value)
            .then(|| value - self.src + self.dest)
    }

    pub fn try_map_range(&self, range: &Range<u64>) -> Option<Range<u64>> {
        let intersect = intersect(&(self.src..self.src + self.size), range);
        if intersect.is_empty() {
            return None;
        } else {
            Some(Range {
                start: intersect.start - self.src + self.dest,
                end: intersect.end - self.src + self.dest,
            })
        }
    }
}

// 52 50 48
impl FromStr for Map {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let values = s
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect::<Vec<u64>>();

        Ok(Map {
            dest: values[0],
            src: values[1],
            size: values[2],
        })
    }
}

fn intersect(left: &Range<u64>, right: &Range<u64>) -> Range<u64> {
    Range {
        start: left.start.max(right.start),
        end: left.end.min(right.end),
    }
}
