use std::io::Write;

#[derive(Debug, PartialEq, Eq, PartialOrd, Clone)]
struct MapRange {
    pub start: i64,
    pub end: i64,
    pub remap_add: i64,
}

impl MapRange {
    pub fn from_input(line: &str) -> MapRange {
        let mut values = line.split_whitespace().map(|n| n.parse::<i64>().unwrap());
        let (to, from, size) = (values.next().unwrap(), values.next().unwrap(), values.next().unwrap());
        MapRange {
            start: from,
            end: from + size,
            remap_add: to - from,
        }
    }

    pub fn try_map(&self, value: i64) -> Option<i64> {
        if value < self.start || value >= self.end {
            None
        } else {
            Some(value + self.remap_add)
        }
    }
}

impl Ord for MapRange {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.start.cmp(&other.start)
    }
}

fn get_maps<'a>(lines: &mut impl Iterator<Item = &'a str>) -> [Vec<MapRange>; 7] {
    let mut maps: [Vec<MapRange>; 7] = [
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
    ];

    let mut current = 0;
    for line in lines.skip(1) {
        match line {
            "seed-to-soil map:" => { current = 0; continue; },
            "soil-to-fertilizer map:" => { current = 1; continue; },
            "fertilizer-to-water map:" => { current = 2; continue; },
            "water-to-light map:" => { current = 3; continue; },
            "light-to-temperature map:" => { current = 4; continue; },
            "temperature-to-humidity map:" => { current = 5; continue; },
            "humidity-to-location map:" => { current = 6; continue; },
            _ => (),
        };

        maps[current].push(MapRange::from_input(line));
    }

    for map_range in maps.iter_mut() {
        map_range.sort_unstable();
    }

    maps
}

pub fn solve_a(input: &String, output: &mut impl Write) {
    let mut lines = input.lines().filter(|l| !l.is_empty());
    let maps = get_maps(&mut lines.clone());
    let seeds = lines.next().unwrap().split_whitespace().skip(1).map(|n| n.parse::<i64>().unwrap());

    let mut lowest = i64::MAX;
    for seed in seeds {
        let mut mapping = seed;
        for mapping_ranges in maps.iter() {
            let mut to_mapping = mapping_ranges.iter().filter_map(|range| range.try_map(mapping));
            if let Some(new_mapping) = to_mapping.next() {
                mapping = new_mapping;
            }
        }

        lowest = i64::min(lowest, mapping);
    }

    write!(output, "{}", lowest).unwrap();
}

pub fn solve_b(input: &String, output: &mut impl Write) {
    let mut lines = input.lines().filter(|l| !l.is_empty());
    let maps = get_maps(&mut lines.clone());
    let mut seed_values = lines.next().unwrap().split_whitespace().skip(1).map(|n| n.parse::<i64>().unwrap());
    let mut seed_ranges = Vec::new();
    loop {
        if let (Some(seed_start), Some(seed_count)) = (seed_values.next(), seed_values.next()) {
            seed_ranges.push(MapRange {
                start: seed_start,
                end: seed_start + seed_count,
                remap_add: 0,
            });
        } else {
            break;
        }
    }
    let mut result = i64::MAX;
    for seed_range in seed_ranges {
        result = i64::min(result, map_lowest(seed_range, &maps));
    }

    write!(output, "{}", result).unwrap();
}

fn map_lowest(map_range: MapRange, maps: &[Vec<MapRange>]) -> i64 {
    if maps.len() == 0 {
        return map_range.start + map_range.remap_add;
    }

    let mut lowest = i64::MAX;
    for mapped_range in map_into(&map_range, maps[0].iter()) {
        lowest = i64::min(lowest, map_lowest(mapped_range, &maps[1..]));
    }

    lowest
}

fn map_into<'a>(map_range: &MapRange, ranges: impl Iterator<Item = &'a MapRange> + 'a) -> Vec<MapRange> {
    let output = MapRange {
        start: map_range.start + map_range.remap_add,
        end: map_range.end + map_range.remap_add,
        remap_add: 0,
    };

    let mut output_ranges = Vec::new();
    for range in ranges {
        if range.end < output.start || range.start > output.end {
            continue;
        }

        output_ranges.push(MapRange {
            start: i64::max(range.start, output.start),
            end: i64::min(range.end, output.end),
            remap_add: range.remap_add,
        });
    }
    let mut last_end = output.start;
    let mut i = 0;
    loop {
        if i >= output_ranges.len() {
            break;
        }

        let range = output_ranges[i].clone();
        if last_end < range.start
        {
            output_ranges.insert(i, MapRange { start: last_end, end: range.start, remap_add: output.remap_add });
            i += 1;
        }
        last_end = range.end;
        i += 1;
    }
    if last_end < output.end {
        output_ranges.push(MapRange { start: last_end, end: output.end, remap_add: output.remap_add });
    }

    output_ranges
}