pub fn _1(input: String) {
    let _input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

    let mut result = 0;
    for line in input.lines() {
        let mut sequence = line
            .trim()
            .split_whitespace()
            .map(|x| x.parse::<i128>().unwrap())
            .collect::<Vec<i128>>();

        let mut last_values = vec![];

        loop {
            let mut new_sequence = vec![];
            last_values.push(sequence[sequence.len() - 1]);
            for i in 1..sequence.len() {
                new_sequence.push(sequence[i] - sequence[i - 1]);
            }
            sequence = new_sequence;
            if sequence.iter().all(|x| x == &0) {
                break;
            }
        }

        let new_values = last_values.iter().rev().fold(0, |acc, x| x + acc);
        result += new_values;
    }
    println!("{}", result);
    assert!(result == 2043677056);
}
pub fn _2(input: String) {
    let _input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

    let mut result = 0;
    for line in input.lines() {
        let mut sequence = line
            .trim()
            .split_whitespace()
            .map(|x| x.parse::<i128>().unwrap())
            .collect::<Vec<i128>>();

        let mut first_values = vec![];

        loop {
            let mut new_sequence = vec![];
            first_values.push(sequence[0]);
            for i in 1..sequence.len() {
                new_sequence.push(sequence[i] - sequence[i - 1]);
            }
            sequence = new_sequence;
            if sequence.iter().all(|x| x == &0) {
                break;
            }
        }

        let new_values = first_values.iter().rev().fold(0, |acc, x| x - acc);

        result += new_values;
    }
    println!("{}", result);
    assert!(result == 1062);
}
