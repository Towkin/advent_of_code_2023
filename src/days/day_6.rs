use std::io::Write;

struct Race {
    pub time: usize,
    pub distance: usize,
}

fn parse_races<'a>(input: &'a String) -> impl Iterator<Item = Race> + 'a {
    let mut number_pairs = input.lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.split_whitespace()
            .skip(1)
            .filter_map(|number| number.parse::<usize>().ok())
        );

    let (times, distances) = (number_pairs.next().unwrap(), number_pairs.next().unwrap());
    times.zip(distances).map(|(time, distance)| Race { time, distance })
}

fn parse_race(input: &String) -> Race {
    let mut number_pairs = input.lines()
        .filter(|line| !line.is_empty())
        .filter_map(|line| line.chars().filter(|c| c.is_ascii_digit())
            .collect::<String>()
            .parse::<usize>().ok());

    let (time, distance) = (number_pairs.next().unwrap(), number_pairs.next().unwrap());
    Race { time, distance }
}

pub fn solve_a(input: &String, output: &mut impl Write) {
    let races = parse_races(input);
    let result = races.map(|race| {
        (0..race.time).filter(|t| (race.time - t) * t > race.distance).count()
    }).fold(1, |acc, v| acc * v);

    write!(output, "{}", result).unwrap();
}

pub fn solve_b(input: &String, output: &mut impl Write) {
    let race = parse_race(input);
    let result = (0..race.time).filter(|t| (race.time - t) * t > race.distance).count();

    write!(output, "{}", result).unwrap();
}
