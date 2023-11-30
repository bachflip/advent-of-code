pub fn _1(input: String) {
    let mut result = 0;
    for line in input.lines() {
        let mut sides = line.split("x");
        let side_1 = sides.next().unwrap().parse::<i32>().unwrap();
        let side_2 = sides.next().unwrap().parse::<i32>().unwrap();
        let side_3 = sides.next().unwrap().parse::<i32>().unwrap();
        let area_1 = side_1 * side_2;
        let area_2 = side_2 * side_3;
        let area_3 = side_3 * side_1;
        result += 2 * (area_1 + area_2 + area_3) + std::cmp::min(area_1, std::cmp::min(area_2, area_3));
    }

    println!("{}", result);
}

pub fn _2(input: String) {
    let mut result = 0;
    for line in input.lines() {
        let mut sides = line.split("x");
        let side_1 = sides.next().unwrap().parse::<i32>().unwrap() * 2;
        let side_2 = sides.next().unwrap().parse::<i32>().unwrap() * 2;
        let side_3 = sides.next().unwrap().parse::<i32>().unwrap() * 2;
        result += side_1 + side_2 + side_3 - std::cmp::max(side_1, std::cmp::max(side_2, side_3)) + (side_1 / 2 * side_2 / 2 * side_3 / 2); 
    }
    println!("{}", result);
}
