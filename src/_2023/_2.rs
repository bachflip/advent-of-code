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
    let mut result = 0;

    let num_balls = |line: &str, color: &str| {
        line.strip_prefix(' ')
            .unwrap()
            .strip_suffix(&format!(" {}", color))
            .unwrap()
            .parse::<i32>()
            .unwrap()
    };

    for line in input.lines() {
        let mut min_red = 0;
        let mut min_green = 0;
        let mut min_blue = 0;

        let drawns = line.split(':').last().unwrap().split(';');
        for drawn in drawns {
            let balls = drawn.split(',');
            balls.for_each(|x| {
                if x.contains("red") {
                    let red = num_balls(x, "red");
                    if red > min_red {
                        min_red = red;
                    }
                }

                if x.contains("blue") {
                    let blue = num_balls(x, "blue");
                    if blue > min_blue {
                        min_blue = blue;
                    }
                }

                if x.contains("green") {
                    let green = num_balls(x, "green");
                    if green > min_green {
                        min_green = green;
                    }
                }
            });
        }
        result += min_red * min_blue * min_green; 
       // println!("{} {} {}", min_red, min_green, min_blue);
    }
    println!("{}", result);
}
