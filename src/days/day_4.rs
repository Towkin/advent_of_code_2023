use std::{io::Write, collections::HashSet};

fn winning_numbers_per_card(input: &String) -> impl Iterator<Item = usize> + '_ {
    let lines = input.lines().filter(|l| !l.is_empty());
    let mut winning_numbers_set = HashSet::with_capacity(10);
    const CARD_LENGTH: usize = "Card ###:".len();
    const WINNING_NUMBERS_LENGTH: usize = " ## ## ## ## ## ## ## ## ## ## |".len();
    const WINNING_NUMBERS_END: usize = CARD_LENGTH + WINNING_NUMBERS_LENGTH;
    lines.map(move |line| {
        let winning_numbers = line[CARD_LENGTH..WINNING_NUMBERS_END - 1]
            .split_whitespace().filter_map(|n| n.parse::<u8>().ok());
        winning_numbers_set.clear();
        for winning_number in winning_numbers {
            winning_numbers_set.insert(winning_number);
        }

        line[WINNING_NUMBERS_END..].split_whitespace().filter_map(|n| n.parse::<u8>().ok()).filter(|n| winning_numbers_set.contains(n)).count()
    })
}

pub fn solve_a(input: &String, output: &mut impl Write) {
    let result: usize = winning_numbers_per_card(input).map(|card_wins| {
        if card_wins < 1 { 0 } else { 1 << card_wins - 1 }
    }).sum();
    write!(output, "{}", result).unwrap();
}

pub fn solve_b(input: &String, output: &mut impl Write) {
    let won_count = winning_numbers_per_card(input).collect::<Vec<_>>();
    let mut hit_count = Vec::with_capacity(won_count.len());
    hit_count.resize(won_count.len(), 0);
    for (i, range) in won_count.iter().enumerate().map(|(i, count)| (i, i + 1..i + count + 1)) {
        hit_count[i] += 1;
        let add_hits = hit_count[i];
        for hit in &mut hit_count[range] {
            *hit += add_hits;
        }
    }
    let result: usize = hit_count.iter().sum();

    write!(output, "{}", result).unwrap();
}