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
pub fn _2(input: String) {}
