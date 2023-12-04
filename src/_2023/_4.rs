use std::collections::{btree_map::BTreeMap, HashMap, HashSet};

pub fn _1(input: String) {
    let _input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    let result = input.lines().fold(0, |acc, line| {
        let (winning, having) = line.split_once(":").unwrap().1.split_once('|').unwrap();
        let winning = winning.trim().split_whitespace().collect::<HashSet<_>>();
        let having = having.trim().split_whitespace().collect::<HashSet<_>>();

        let count = having.intersection(&winning).count();

        let points = if count == 0 {
            0
        } else {
            2_i32.pow((count - 1).try_into().unwrap())
        };
        acc + points
    });
    println!("{}", result);
}
pub fn _2(input: String) {
    let _input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    let mut cards = vec![1; input.lines().count()];

    for line in input.lines() {
        let (card, numbers) = line.split_once(":").unwrap();
        let card_number = card
            .split_whitespace()
            .last()
            .unwrap()
            .parse::<usize>()
            .unwrap();

        let (winning, having) = numbers.split_once('|').unwrap();
        let nums = |it: &str| {
            it.trim()
                .split_whitespace()
                .map(|x| x.parse::<u32>().unwrap())
                .collect::<HashSet<_>>()
        };

        let won = nums(having).intersection(&nums(winning)).count();
        let current_cards_won = cards[card_number - 1];
        for i in card_number..(card_number + won) {
            cards[i] += current_cards_won;
        }
    }

    let result: u32 = cards.iter().sum();
    println!("{}", result);
    assert!(result == 5921508);
}
