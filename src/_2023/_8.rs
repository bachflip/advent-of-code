use std::collections::HashMap;

pub fn _1(input: String) {
    let _input = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";

    let _input = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";
    let (direction, map) = input.split_once("\n\n").unwrap();
    let direction = direction.chars().collect::<Vec<_>>();
    let map = map
        .lines()
        .map(|line| {
            println!("-- {}", line);
            let (k, m) = line.split_once('=').unwrap();
            let k = k.trim();
            let (l, r) = m
                .trim()
                .trim_matches(|c| c == '(' || c == ')')
                .split_once(", ")
                .unwrap();
            (k, (l, r))
        })
        .collect::<HashMap<_, _>>();

    println!("{:#?}", map);

    let mut result = 0;
    let (mut l, mut r) = map.get("AAA").unwrap();
    let mut i = 0;
    loop {
        result += 1;
        let c = direction[i];
        let next = if c == 'L' { l } else { r };
        println!("{}", next);
        if next == "ZZZ" {
            break;
        }
        (l, r) = *map.get(next).unwrap();
        if i + 1 == direction.len() {
            i = 0;
        } else {
            i += 1;
        }
    }
    println!("{}", result);
}
pub fn _2(input: String) {
    let _input = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";
    let (direction, map) = input.split_once("\n\n").unwrap();
    let direction = direction.chars().collect::<Vec<char>>();
    let map = map
        .lines()
        .map(|line| {
            let (k, m) = line.split_once('=').unwrap();
            let k = k.trim().to_owned();
            let (l, r) = m
                .trim()
                .trim_matches(|c| c == '(' || c == ')')
                .split_once(", ")
                .unwrap();
            (k, (l.to_owned(), r.to_owned()))
        })
        .collect::<HashMap<_, _>>();

    let mut nodes = map
        .keys()
        .filter(|k| k.ends_with('A'))
        .map(|k| k.to_string())
        .collect::<Vec<String>>();

    let mut steps = vec![0; nodes.len()];

    let mut result = 0;
    let mut i = 0;
    loop {
        result += 1;
        let c = direction[i];

        for (node, j) in nodes.iter_mut().zip(steps.iter_mut()) {
            let path = map.get(node).unwrap().clone();
            *node = if c == 'L' { path.0 } else { path.1 };
            if node.ends_with('Z') && *j == 0 {
                *j = result;
                println!("{:#?}", j);
            }
        }

        if steps.iter().all(|x| *x != 0) {
            println!("{:#?}", steps);
            break;
        }

        if i + 1 == direction.len() {
            i = 0;
        } else {
            i += 1;
        }
    }
    let result = lowest_common_multiple(steps);
    println!("{}", result);
    assert!(result == 13_830_919_117_339);
}

fn lowest_common_multiple(numbers: Vec<u64>) -> u64 {
    let mut i = 1;
    let mut multiple;
    loop {
        multiple = i * numbers[0];
        if numbers.iter().all(|x| multiple % x == 0) {
            break;
        }
        i += 1;
    }
    multiple
}
