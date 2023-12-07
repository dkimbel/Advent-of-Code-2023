use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd)]
enum HandType {
    HighCard,
    Pair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl HandType {
    fn from_cards(cards: &[Card; 5]) -> HandType {
        let mut counts_by_card: HashMap<&Card, usize> = HashMap::new();
        for card in cards {
            *counts_by_card.entry(card).or_insert(0) += 1;
        }
        let joker_count = counts_by_card.get(&Card::Joker).cloned().unwrap_or(0);
        counts_by_card.remove(&Card::Joker);
        let mut card_counts = counts_by_card.values().collect::<Vec<_>>();
        // sort in descending order
        card_counts.sort_by(|a, b| b.cmp(a));
        let max_card_count = if !card_counts.is_empty() {
            card_counts[0] + joker_count
        } else {
            joker_count
        };
        use HandType::*;
        match max_card_count {
            5 => FiveOfAKind,
            4 => FourOfAKind,
            3 => {
                let secondary_card_count = card_counts[1];
                match secondary_card_count {
                    2 => FullHouse,
                    _ => ThreeOfAKind,
                }
            }
            2 => {
                let secondary_card_count = card_counts[1];
                match secondary_card_count {
                    2 => TwoPair,
                    _ => Pair,
                }
            }
            1 => HighCard,
            _ => panic!("Unexpected card count {}", max_card_count),
        }
    }

    fn to_num(self) -> usize {
        use HandType::*;
        match self {
            HighCard => 1,
            Pair => 2,
            TwoPair => 3,
            ThreeOfAKind => 4,
            FullHouse => 5,
            FourOfAKind => 6,
            FiveOfAKind => 7,
        }
    }
}

impl Ord for HandType {
    fn cmp(&self, other: &Self) -> Ordering {
        self.to_num().cmp(&other.to_num())
    }
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, PartialOrd)]
enum Card {
    Joker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl Card {
    fn from_char(c: char, jacks_are_jokers: bool) -> Card {
        use Card::*;
        match c {
            '2' => Two,
            '3' => Three,
            '4' => Four,
            '5' => Five,
            '6' => Six,
            '7' => Seven,
            '8' => Eight,
            '9' => Nine,
            'T' => Ten,
            'J' => {
                if jacks_are_jokers {
                    Joker
                } else {
                    Jack
                }
            }
            'Q' => Queen,
            'K' => King,
            'A' => Ace,
            _ => panic!("No card for char {}", c),
        }
    }

    fn to_num(self) -> usize {
        use Card::*;
        match self {
            Joker => 1,
            Two => 2,
            Three => 3,
            Four => 4,
            Five => 5,
            Six => 6,
            Seven => 7,
            Eight => 8,
            Nine => 9,
            Ten => 10,
            Jack => 11,
            Queen => 12,
            King => 13,
            Ace => 14,
        }
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> Ordering {
        self.to_num().cmp(&other.to_num())
    }
}

#[derive(Debug)]
struct Hand {
    cards: [Card; 5],
    hand_type: HandType,
    bid: usize,
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let hand_type_cmp = self.hand_type.cmp(&other.hand_type);
        if hand_type_cmp == Ordering::Equal {
            for (self_card, other_card) in std::iter::zip(self.cards, &other.cards) {
                let card_cmp = self_card.cmp(other_card);
                if card_cmp != Ordering::Equal {
                    return card_cmp;
                }
            }
        } else {
            return hand_type_cmp;
        }
        Ordering::Equal
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.hand_type == other.hand_type && self.cards == other.cards
    }
}

impl Eq for Hand {}

fn solve(file_path: &str, jacks_are_jokers: bool) -> usize {
    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);
    let mut hands: Vec<Hand> = Vec::new();
    for line in reader.lines() {
        let line_content = &line.unwrap();
        let split = line_content.split(" ").collect::<Vec<_>>();
        let bid = split[1].parse::<usize>().unwrap();
        let cards_vec = split[0]
            .chars()
            .map(|c| Card::from_char(c, jacks_are_jokers))
            .collect::<Vec<_>>();
        let cards = [
            cards_vec[0],
            cards_vec[1],
            cards_vec[2],
            cards_vec[3],
            cards_vec[4],
        ];
        let hand_type = HandType::from_cards(&cards);
        hands.push(Hand {
            cards,
            hand_type,
            bid,
        });
    }
    // sort from worst hand to best
    hands.sort();
    let mut hand_winnings: Vec<usize> = Vec::new();

    for (i, hand) in hands.iter().enumerate() {
        let hand_num: usize = i + 1;
        hand_winnings.push(hand_num * hand.bid);
    }
    hand_winnings.iter().sum::<usize>()
}

fn main() {
    let file_path = "resources/input_1";

    let part_1_solution = solve(file_path, false);
    println!("Part 1 solution: {part_1_solution}");

    let part_2_solution = solve(file_path, true);
    println!("Part 2 solution: {part_2_solution}");
}
