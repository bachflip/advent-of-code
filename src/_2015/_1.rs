pub fn _1(input: String) {
    let mut floor = 0;
    for c in input.chars() {
        if c == '(' {
            floor += 1;
        } 
        if c == ')' {
            floor -= 1;
        }
    }
    println!("{}", floor);
}

pub fn _2(input: String) {
    println!("hello from day 2 of year 2015");
}


