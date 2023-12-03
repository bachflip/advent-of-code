use std::collections::{btree_map::BTreeMap, HashSet};

pub fn _1(input: String) {
    let _input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    let mut split = input.split_whitespace();
    let height = split.clone().count();
    let width = split.clone().next().unwrap().len();

    let adjacents_to_number = |i: usize, number: u32| {
        let number_length = number.to_string().len();
        let mut result = HashSet::new();

        let not_top = i >= width;
        let not_bottom = i < width * (height - 1);
        let not_left = i % width > 0;
        let not_right = i % width < width - 1;

        for idx in 0..number_length {
            if not_top {
                // not top row
                result.insert(i + idx - width); // add right above
            }

            if not_bottom {
                // not bottom row
                result.insert(i + idx + width); // add right below
            }
        }

        if not_left {
            // not left most column
            result.insert(i - 1); // add left
            if not_top {
                result.insert(i - width - 1); // add upper left
            }
            if not_bottom {
                result.insert(i + width - 1); // add lower left
            }
        }

        if not_right {
            // not right most column
            result.insert(i + number_length); // add right
            if not_top {
                result.insert(i + number_length - width); // add upper right
            }
            if not_bottom {
                result.insert(i + number_length + width); // add lower right
            }
        }

        result
    };

    let mut numbers = BTreeMap::new(); // Do we need to preserve insert order?

    let mut current_number = 0;
    let mut current_index = usize::MAX;
    for (i, c) in input.replace("\n", "").chars().enumerate() {
        if !c.is_numeric() {
            current_number = 0;
            current_index = usize::MAX;
        } else {
            if current_index == usize::MAX {
                current_index = i;
            }
            current_number = current_number * 10 + c.to_digit(10).unwrap();
            numbers.insert(current_index, current_number);
        }
    }

    let symbol_indices = input.replace("\n", "").char_indices().filter_map(|c| {
        if !c.1.is_numeric() && c.1 != '.' {
            Some(c.0)
        } else {
            None
        }
    }).collect::<HashSet<usize>>();

    let mut result = 0;
    for (k, v) in numbers.iter() {
        let adjacents = adjacents_to_number(*k, *v);
        if adjacents.intersection(&symbol_indices).count() > 0 {
            result += *v;
        }
    }

    println!("{:#?}", result);
}

pub fn _2(input: String) {}
