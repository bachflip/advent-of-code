pub fn _1(input: String) {
    let _input = "Time:      7  15   30
        Distance:  9  40  200";

    let mut time_distance = vec![];
    for line in input.lines() {
        let parsed = line
            .split_once(":")
            .unwrap()
            .1
            .trim()
            .split_whitespace()
            .map(|x| x.parse::<u32>().unwrap())
            .collect::<Vec<_>>();
        time_distance.push(parsed);
    }
    let races = time_distance[0]
        .iter()
        .zip(&time_distance[1])
        .collect::<Vec<_>>();

    let result = races.iter().fold(1, |acc, (time, distance)| {
        let mut winning = 0;
        for i in 1..**time {
            if i * (*time - i) > **distance {
                winning += 1;
            }
        }
        winning * acc
    });

    println!("{}", result);
}
pub fn _2(input: String) {
    let _input = "Time:      7  15   30
Distance:  9  40  200";

    let mut time_distance = vec![];
    for line in input.lines() {
        let parsed = line
            .split_once(":")
            .unwrap()
            .1
            .trim()
            .replace(" ", "")
            .parse::<u64>()
            .unwrap();
        time_distance.push(parsed);
    }
    let time = time_distance[0];
    let distance = time_distance[1];
    let result;
    let mut winning_left = u64::MAX;
    let mut winning_right = u64::MAX;

    for i in 0..time {
        if i * (time - i) > distance {
            winning_left = i;
            break;
        }
    }
    for i in (0..time).rev() {
        if i * (time - i) > distance {
            winning_right = i;
            break;
        }
    }
    println!("{}", winning_left);
    println!("{}", winning_right);

    result = winning_right + 1 - winning_left;

    println!("{}", result);
    assert!(result == 39570185);
}
