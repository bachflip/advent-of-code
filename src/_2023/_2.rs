pub fn _1(input: String) {
    let red_limit = 12;
    let green_limit = 13;
    let blue_limit = 14;

    let mut result = 0;

    for line in input.lines() {
        let id = line
            .split(':')
            .next()
            .unwrap()
            .split(' ')
            .last()
            .unwrap()
            .parse::<i32>()
            .unwrap();
        let drawns = line.split(':').last().unwrap().split(';');
        let mut game_valid = true;
        for drawn in drawns {
            let balls = drawn.split(',');
            let valid = balls.fold(true, |acc, x| {
                if x.contains("red") {
                    let red = x
                        .strip_prefix(' ')
                        .unwrap()
                        .strip_suffix(" red")
                        .unwrap()
                        .parse::<i32>()
                        .unwrap();
                    acc && red <= red_limit
                } else if x.contains("blue") {
                    let blue = x
                        .strip_prefix(' ')
                        .unwrap()
                        .strip_suffix(" blue")
                        .unwrap()
                        .parse::<i32>()
                        .unwrap();
                    acc && blue <= blue_limit
                } else if x.contains("green") {
                    let green = x
                        .strip_prefix(' ')
                        .unwrap()
                        .strip_suffix(" green")
                        .unwrap()
                        .parse::<i32>()
                        .unwrap();
                    acc && green <= green_limit
                } else {
                    acc
                }
            });
            game_valid = game_valid && valid;
        }
        if game_valid {
            result += id;
        }
    }
    println!("{}", result);
}

pub fn _2(input: String) {
    let result = input.lines().fold(0, |acc, line| {
        acc + line
            .split(':')
            .last()
            .unwrap()
            .replace(';', ",")
            .split(',')
            .map(|color| color.strip_prefix(' ').unwrap().split_once(' ').unwrap())
            .map(|(num, color)| (num.parse::<i32>().unwrap(), color))
            .fold(vec![0; 3], |a, (n, c)| {
                (0..3)
                    .map(|i| std::cmp::max(a[i], ((c.len() - 3 == i) as i32) * n))
                    .collect()
            })
            .iter()
            .product::<i32>()
    });

    println!("{}", result);
}
