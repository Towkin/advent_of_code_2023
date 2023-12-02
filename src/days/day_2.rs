use std::io::Write;

pub fn solve_a(input: &String, output: &mut impl Write) {
    const MAX_COUNT_PER_COLOR: [u32; 3] = [
        12,
        13,
        14,
    ];
    let lines = input.lines().filter(|l| !l.is_empty());
    let result: u32 = lines.map(|line| {
        let (game, cubes) = line.split_once(':').unwrap();
        let game = game["Game ".len()..].parse::<u32>().unwrap();
        let impossible = cubes.split([',', ';']).any(|cube_count_and_color| {
            let (count, color) = cube_count_and_color.trim_start().split_once(' ').unwrap();
            let count = count.parse::<u32>().unwrap();
            count > MAX_COUNT_PER_COLOR[match color {
                "red" => 0,
                "green" => 1,
                "blue" => 2,
                _ => panic!(),
            }]
        });

        if impossible { 0 }
        else { game }
    }).sum();
    write!(output, "{}", result).unwrap();
}

pub fn solve_b(input: &String, output: &mut impl Write) {
    let lines = input.lines().filter(|l| !l.is_empty());
    let result: u32 = lines.map(|line| {
        let (_, cubes) = line.split_once(':').unwrap();
        let mut min_cubes = [0, 0, 0];

        for cube_count_and_color in cubes.split([',', ';']) {
            let (count, color) = cube_count_and_color.trim_start().split_once(' ').unwrap();
            let count = count.parse::<u32>().unwrap();
            let cube_index = match color {
                "red" => 0,
                "green" => 1,
                "blue" => 2,
                _ => panic!(),
            };
            if min_cubes[cube_index] < count {
                min_cubes[cube_index] = count;
            }
        }

        min_cubes[0] * min_cubes[1] * min_cubes[2]
    }).sum();
    write!(output, "{}", result).unwrap();
}
