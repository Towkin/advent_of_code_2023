use std::io::Write;

#[derive(PartialEq, Eq, Debug)]
enum MapData {
    None,
    Symbol,
    Digit(u8),
}

pub fn solve_a(input: &String, output: &mut impl Write) {
    let lines = input.lines().filter(|line| !line.is_empty());
    let size_x = input.find('\n').unwrap();
    let size_y = lines.clone().count();
    let mut map: Vec<MapData> = lines.map(|line| line.chars().map(|c| match c {
        '.' => MapData::None,
        '0'..='9' => MapData::Digit(c as u8 - '0' as u8),
        _ => MapData::Symbol,
    })).flatten().collect();

    assert_eq!(map.len(), size_x * size_y);

    let mut result = 0;
    let map_positions = (0..size_y).flat_map(|y| (0..size_x).map(move |x| (x, y)));

    for (x, y) in map_positions {
        if map[y * size_x + x] != MapData::Symbol {
            continue;
        }

        let range_y = (y - 1).max(0)..(y + 2).min(size_y);
        let neighbors = range_y.flat_map(|y| {
            let range_x = (x - 1).max(0)..(x + 2).min(size_x);
            range_x.map(move |x| (x, y))
        });
        for (x, y) in neighbors {
            let line_start_index = y * size_x;
            if match map[line_start_index + x] {
                MapData::Digit(_) => false,
                _ => true
            } {
                continue;
            }

            let previous_data = &map[line_start_index..line_start_index + x];
            let first_index = previous_data.iter().rposition(|d| match d {
                MapData::Digit(_) => false,
                _ => true,
            }).and_then(|p| Some(p + 1)).unwrap_or(0) + line_start_index;
            let next_data = &map[line_start_index + x..line_start_index + size_x];
            let last_index = next_data.iter().position(|d| match d {
                MapData::Digit(_) => false,
                _ => true,
            }).and_then(|p| Some(p + x)).unwrap_or(size_x) + line_start_index;

            let value = map[first_index..last_index].iter().fold(0, |number, d| match d {
                MapData::Digit(digit) => number * 10 + *digit as i32,
                _ => number,
            });
            result += value;

            for i in first_index..last_index {
                map[i] = MapData::None;
            }
        }
    }

    write!(output, "{}", result).unwrap();
}

pub fn solve_b(input: &String, output: &mut impl Write) {
    let lines = input.lines().filter(|line| !line.is_empty());
    let size_x = input.find('\n').unwrap();
    let size_y = lines.clone().count();
    let mut map: Vec<MapData> = lines.map(|line| line.chars().map(|c| match c {
        '.' => MapData::None,
        '0'..='9' => MapData::Digit(c as u8 - '0' as u8),
        _ => MapData::Symbol,
    })).flatten().collect();

    assert_eq!(map.len(), size_x * size_y);

    let mut result = 0;
    let map_positions = (0..size_y).flat_map(|y| (0..size_x).map(move |x| (x, y)));

    for (x, y) in map_positions {
        if map[y * size_x + x] != MapData::Symbol {
            continue;
        }

        let range_y = (y - 1).max(0)..(y + 2).min(size_y);
        let neighbors = range_y.flat_map(|y| {
            let range_x = (x - 1).max(0)..(x + 2).min(size_x);
            range_x.map(move |x| (x, y))
        });
        let mut ratio_1 = None;
        let mut ratio_2 = None;
        for (x, y) in neighbors {
            let line_start_index = y * size_x;
            if match map[line_start_index + x] {
                MapData::Digit(_) => false,
                _ => true
            } {
                continue;
            }

            assert!(ratio_1 == None || ratio_2 == None);

            let previous_data = &map[line_start_index..line_start_index + x];
            let first_index = previous_data.iter().rposition(|d| match d {
                MapData::Digit(_) => false,
                _ => true,
            }).and_then(|p| Some(p + 1)).unwrap_or(0) + line_start_index;
            let next_data = &map[line_start_index + x..line_start_index + size_x];
            let last_index = next_data.iter().position(|d| match d {
                MapData::Digit(_) => false,
                _ => true,
            }).and_then(|p| Some(p + x)).unwrap_or(size_x) + line_start_index;

            let value = map[first_index..last_index].iter().fold(0, |number, d| match d {
                MapData::Digit(digit) => number * 10 + *digit as i32,
                _ => number,
            });
            for i in first_index..last_index {
                map[i] = MapData::None;
            }
            if ratio_1 == None {
                ratio_1 = Some(value);
            } else if ratio_2 == None {
                ratio_2 = Some(value);
            }
        }

        if let (Some(r1), Some(r2)) = (ratio_1, ratio_2) {
            result += r1 * r2;
        }
    }
    write!(output, "{}", result).unwrap();
}