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
pub fn _2(input: String) {}
