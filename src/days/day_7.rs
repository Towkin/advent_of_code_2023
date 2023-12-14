use std::{io::Write, cmp::Ordering};

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord)]
enum HandType {
    None,
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, PartialOrd, Ord)]
enum Card {
    Joker,
    Value2,
    Value3,
    Value4,
    Value5,
    Value6,
    Value7,
    Value8,
    Value9,
    Value10,
    Queen,
    King,
    Ace,
}

#[derive(Debug, Eq, PartialEq, PartialOrd)]
struct Hand<T> {
    hand_type: HandType,
    cards: [T; 5],
}

impl<T: Ord> Ord for Hand<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.hand_type.cmp(&other.hand_type) {
            Ordering::Equal => self.cards.cmp(&other.cards),
            ordering => ordering,
        }
    }
}

fn new_hand_a(input: &str) -> Hand<u8> {
    assert_eq!(input.len(), 5);
    let mut values = input.chars().map(card_value_a);
    let cards = [
        values.next().unwrap(),
        values.next().unwrap(),
        values.next().unwrap(),
        values.next().unwrap(),
        values.next().unwrap(),
    ];
    Hand {
        hand_type: get_hand_type_a(cards),
        cards,
    }
}

fn new_hand_b(input: &str) -> Hand<Card> {
    assert_eq!(input.len(), 5);
    let mut values = input.chars().map(card_value_b);
    let cards = [
        values.next().unwrap(),
        values.next().unwrap(),
        values.next().unwrap(),
        values.next().unwrap(),
        values.next().unwrap(),
    ];
    Hand {
        hand_type: get_hand_type_b(cards),
        cards,
    }
}

fn equal_count<T: Eq>(cards: &[T]) -> usize
{
    (0..cards.len()).flat_map(|i| (i + 1..cards.len()).filter(move |k| cards[i] == cards[*k])).count()
}

fn get_hand_type_a<T: Copy + Eq>(cards: [T; 5]) -> HandType {
    match equal_count(&cards) {
        10 => HandType::FiveOfAKind,
        6 => HandType::FourOfAKind,
        4 => HandType::FullHouse,
        3 => HandType::ThreeOfAKind,
        2 => HandType::TwoPair,
        1 => HandType::OnePair,
        0 => HandType::HighCard,
        _ => HandType::None,
    }
}

fn card_value_a(card: char) -> u8 {
    match card {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 11,
        'T' => 10,
        '9' => 9,
        '8' => 8,
        '7' => 7,
        '6' => 6,
        '5' => 5,
        '4' => 4,
        '3' => 3,
        '2' => 2,
        _ => 0,
    }
}

fn get_hand_type_b(cards: [Card; 5]) -> HandType {
    let mut sorted_cards: [Card; 5] = [Card::Joker; 5];
    sorted_cards.clone_from_slice(&cards);
    sorted_cards.sort_unstable_by(|a, b| b.cmp(a));
    let sorted_cards = sorted_cards;
    let non_joker_count = sorted_cards.iter().filter(|c| **c != Card::Joker).count();
    let joker_count = 5 - non_joker_count;
    match joker_count {
        0 => get_hand_type_a(cards),
        1 => match equal_count(&sorted_cards[0..4]) {
            0 => HandType::OnePair,
            1 => HandType::ThreeOfAKind,
            2 => HandType::FullHouse,
            3 => HandType::FourOfAKind,
            6 => HandType::FiveOfAKind,
            _ => HandType::None,
        },
        2 => match equal_count(&sorted_cards[0..3]) {
            0 => HandType::ThreeOfAKind,
            1 => HandType::FourOfAKind,
            3 => HandType::FiveOfAKind,
            _ => HandType::None,
        },
        3 => match sorted_cards[0] == sorted_cards[1] {
            true => HandType::FiveOfAKind,
            false => HandType::FourOfAKind,
        },
        4 => HandType::FiveOfAKind,
        5 => HandType::FiveOfAKind,
        _ => HandType::None,
    }
}

fn card_value_b(card: char) -> Card {
    match card {
        'A' => Card::Ace,
        'K' => Card::King,
        'Q' => Card::Queen,
        'T' => Card::Value10,
        '9' => Card::Value9,
        '8' => Card::Value8,
        '7' => Card::Value7,
        '6' => Card::Value6,
        '5' => Card::Value5,
        '4' => Card::Value4,
        '3' => Card::Value3,
        '2' => Card::Value2,
        'J' => Card::Joker,
        _ => panic!(),
    }
}

pub fn solve_a(input: &String, output: &mut impl Write) {
    let mut hands_and_bids: Vec<_> = input.lines()
        .filter(|line| !line.is_empty())
        .filter_map(|line| line.split_once(' '))
        .map(|(hand, bid)| (new_hand_a(hand), bid.parse::<usize>().unwrap()))
        .collect();
    hands_and_bids.sort_unstable_by(|(hand_a, _), (hand_b, _)| hand_a.cmp(hand_b));
    let result: usize = hands_and_bids.iter().enumerate().map(|(i, (_, bid))| (i + 1) * bid).sum();
    write!(output, "{}", result).unwrap();
}

pub fn solve_b(input: &String, output: &mut impl Write) {
    let mut hands_and_bids: Vec<_> = input.lines()
        .filter(|line| !line.is_empty())
        .filter_map(|line| line.split_once(' '))
        .map(|(hand, bid)| (new_hand_b(hand), bid.parse::<usize>().unwrap()))
        .collect();
    hands_and_bids.sort_unstable_by(|(hand_a, _), (hand_b, _)| hand_a.cmp(hand_b));
    let result: usize = hands_and_bids.iter().enumerate().map(|(i, (_, bid))| (i + 1) * bid).sum();
    write!(output, "{}", result).unwrap();
}