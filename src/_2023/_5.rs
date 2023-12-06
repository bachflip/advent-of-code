use std::collections::{btree_map::BTreeMap, BTreeSet, VecDeque};

pub fn _1(input: String) {
    let _input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

    let mut sections = input.split("\n\n");
    let mut seeds = sections
        .next()
        .unwrap()
        .split_once(":")
        .unwrap()
        .1
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    for section in sections {
        let maps = section.trim().split_once(":\n").unwrap().1;
        for seed in seeds.iter_mut() {
            for map in maps.split("\n") {
                let mut map_elems = map.split_whitespace();
                let des = map_elems.next().unwrap().parse::<u64>().unwrap();
                let src = map_elems.next().unwrap().parse::<u64>().unwrap();
                let range = map_elems.next().unwrap().parse::<u64>().unwrap();

                if *seed >= src && *seed < src + range {
                    *seed = des + (*seed - src);
                    break;
                }
            }
        }
    }
    let result = seeds.iter().min().unwrap().to_owned();
    println!("{}", result);
    assert!(result == 1181555926);
}
pub fn _2(input: String) {
    let _input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

    let mut sections = input.split("\n\n");
    let binding = sections
        .next()
        .unwrap()
        .split_once(":")
        .unwrap()
        .1
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();
    let mut seeds = binding.chunks(2).map(|x| (x[0], x[1])).collect::<Vec<_>>();

    for section in sections {
        let maps = section.trim().split_once(":\n").unwrap().1;
        let mappings = maps
            .split("\n")
            .map(|x| {
                x.trim()
                    .split_whitespace()
                    .map(|y| y.parse::<u64>().unwrap())
                    .collect::<Vec<_>>()
            })
            .map(|x| (x[0], x[1], x[2]))
            .collect::<Vec<_>>();

        seeds = map_range(seeds.clone(), mappings);
    }
    let result = seeds.iter().map(|x| x.0).min().unwrap();
    println!("{}", result);
    assert!(result == 37806486);
}

fn map_range(input: Vec<(u64, u64)>, mapping: Vec<(u64, u64, u64)>) -> Vec<(u64, u64)> {
    let mut input = input.into_iter().collect::<VecDeque<_>>();

    let mut output = Vec::new();

    loop {
        if input.len() == 0 {
            break;
        }

        let mut src = Some(input.pop_front().unwrap());
        for map in mapping.iter() {
            let result = split_range(src.unwrap(), *map);
            if result.len() == 0 {
                continue;
            } else if result.len() == 1 {
                output.push(result[0]);
            } else if result.len() == 2 {
                output.push(result[0]);
                input.push_back(result[1]);
            } else if result.len() == 3 {
                output.push(result[0]);
                input.push_back(result[1]);
                input.push_back(result[2]);
            } else {
                panic!("this should not happen");
            }
            src = None;
            break;
        }

        if src.is_some() {
            output.push(src.unwrap());
        }
    }

    output.into()
}

fn split_range(src: (u64, u64), map: (u64, u64, u64)) -> Vec<(u64, u64)> {
    let (a, x) = src;
    let (b, y) = (map.1, map.2);
    let des = map.0;

    let mut result = vec![];
    if a + x <= b || b + y <= a {
        // continue
    } else if a < b && b < a + x && a + x <= b + y {
        let mapped = (des, a + x - b);
        let unmapped = (a, b - a);
        result.append(&mut vec![mapped, unmapped]);
    } else if a >= b && b + y > a && a + x > b + y {
        let mapped = (a - b + des, b + y - a);
        let unmapped = (b + y, a + x - b - y);
        result.append(&mut vec![mapped, unmapped]);
    } else if a >= b && a + x <= b + y {
        let mapped = (a - b + des, x);
        result.append(&mut vec![mapped]);
    } else if b >= a && b + y <= a + x {
        let mapped = (des, y);
        let unmapped_1 = (a, b - a);
        let ummapped_2 = (b + y, a + x - b - y);
        result.append(&mut vec![mapped, unmapped_1, ummapped_2]);
    } else {
        panic!("this should not happen");
    }
    result
}

//#[cfg(test)]
//mod test_split_range {
//    use super::*;
//    use pretty_assertions::{assert_eq, assert_ne};
//
//    #[test]
//    fn test() {
//        let input: Vec<((u64, u64), (u64, u64, u64))> = vec![
//            ((1, 2), (5, 1, 2)),
//            ((1, 3), (5, 2, 3)),
//            ((1, 4), (5, 2, 2)),
//            ((2, 2), (5, 1, 5)),
//            ((2, 3), (5, 1, 3)),
//        ];
//
//        let result = [
//            vec![(1 + 5, 2)],
//            vec![(2 + 5, 2), (1, 1)],
//            vec![(2 + 5, 2), (1, 1), (4, 1)],
//            vec![(2 + 5, 2)],
//            vec![(2 + 5, 2), (4, 1)],
//        ];
//        for i in 0..input.len() {
//            assert_eq!(split_range(input[i].0, input[i].1), result[i]);
//        }
//    }
//}
