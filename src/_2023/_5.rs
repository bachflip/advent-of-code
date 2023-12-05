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
    let input = "seeds: 79 14 55 13

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

    //let mut mapped = vec![];
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

        map_range(seeds.clone(), mappings);
        break;
    }
    //let result = seeds.iter().min().unwrap().to_owned();
    //    println!("{}", result);
    //   assert!(result == 1181555926);
}

fn map_range(input: Vec<(u64, u64)>, mapping: Vec<(u64, u64, u64)>) -> Vec<(u64, u64)> {
    let mapping = mapping.into_iter().collect::<VecDeque<_>>();

    let mut input = input.into_iter().collect::<VecDeque<_>>();
    let mut output = VecDeque::new();

    loop {
        let input_len = input.len();
        if input_len == 0 {
            break;
        }

        let mut src = input.pop_front().unwrap();
        let it = mapping.iter();
        for map in it {
            let result = split_range(src, *map);
            if result.len() == 0 {
                continue;
            } else if result.len() == 1 {
                output.push_back(result[0]);
            } else if result.len() == 2 {
                output.push_back(result[0]);
                input.push_back(result[1]);
            } else {
                output.push_back(result[0]);
                input.push_back(result[1]);
                input.push_back(result[2]);
            }
            src = (0, 0);
        }
        if src != (0, 0) {
            output.push_back(src);
        }
    }

    println!("{:#?}", input);
    println!("{:#?}", output);

    vec![]
}

fn split_range(src: (u64, u64), map: (u64, u64, u64)) -> Vec<(u64, u64)> {
    let (a, x) = src;
    let (b, y) = (map.1, map.2);
    let des = map.0;

    if a + x <= b || b + y <= a {
        vec![]
    } else if a <= b && b < a + x && a + x < b + y {
        let mapped = (des, a + x - b);
        let unmapped = (a, b - a);
        println!("x");
        println!("map {:#?}", map);
        println!("src {:#?}", src);
        println!("result {:#?}", mapped);

        vec![mapped, unmapped]
    } else if a >= b && b + y > a && a + x > b + y {
        let mapped = (a - b + des, b + y - a);
        let unmapped = (b + y, a + x - b - y);
        println!("xx");
        println!("src {:#?}", src);
        println!("map {:#?}", map);
        println!("result {:#?}", mapped);
        vec![mapped, unmapped]
    } else if a >= b && a + x <= b + y {
        let mapped = (a - b + des, x);
        println!("xxx");
        println!("src {:#?}", src);
        println!("map {:#?}", map);
        println!("result {:#?}", mapped);
        vec![mapped]
    } else if b >= a && b + y <= a + x {
        let mapped = (des, y);
        let unmapped_1 = (a, b - a);
        let ummapped_2 = (b + y, a + x - b - y);
        println!("xxxx");
        println!("src {:#?}", src);
        println!("map {:#?}", map);
        println!("result {:#?}", mapped);
        vec![mapped, unmapped_1, ummapped_2]
    } else {
        vec![]
    }
}

#[cfg(test)]
mod test_split_range {
    use super::*;
    use pretty_assertions::{assert_eq, assert_ne};

    #[test]
    fn test() {
        let input: Vec<((u64, u64), (u64, u64, u64))> = vec![
            ((1, 2), (5, 1, 2)),
            ((1, 3), (5, 2, 3)),
            ((1, 4), (5, 2, 2)),
            ((2, 2), (5, 1, 5)),
            ((2, 3), (5, 1, 3)),
        ];

        let result = [
            vec![(1 + 5, 2)],
            vec![(2 + 5, 2), (1, 1)],
            vec![(2 + 5, 2), (1, 1), (4, 1)],
            vec![(2 + 5, 2)],
            vec![(2 + 5, 2), (4, 1)],
        ];
        for i in 0..input.len() {
            assert_eq!(split_range(input[i].0, input[i].1), result[i]);
        }
    }
}
