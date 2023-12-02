use std::io::Write;

pub fn solve_a(input: &String, output: &mut impl Write) {
    let lines = input.lines().filter(|l| !l.is_empty());
    let result: u32 = lines.map(|line| {
        let mut numbers = line.chars().filter_map(|c| c.to_digit(10));
        let b = numbers.clone().next_back().unwrap();
        let a = numbers.next().unwrap();
        a * 10 + b
    }).sum();

    write!(output, "{}", result).unwrap();
}

const TEXT_DIGITS: [&'static str; 9]= [
    "one",
    "two",
    "three",
    "four",
    "five",
    "six",
    "seven",
    "eight",
    "nine"
];

pub fn solve_b(input: &String, output: &mut impl Write) {
    let lines = input.lines().filter(|l| !l.is_empty());
    let result: u32 = lines.map(|line| {
        let mut numbers = line.char_indices().filter_map(|(i, c)| {
            let part = &line[i..line.len()];
            let to_text_digit = TEXT_DIGITS
                .iter()
                .position(|text_digit| part.starts_with(text_digit))
                .and_then(|text_index| Some(text_index as u32 + 1));

            char::to_digit(c, 10).or_else(|| to_text_digit)
        });
        let b = numbers.clone().next_back().unwrap();
        let a = numbers.next().unwrap();
        a * 10 + b
    }).sum();

    write!(output, "{}", result).unwrap();
}
