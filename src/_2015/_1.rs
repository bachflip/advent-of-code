pub fn _1(input: String) {
    let floor = input.chars().fold(0, |acc, x| {
        if x == '(' {
            acc + 1
        } else if x == ')' {
            acc - 1
        } else {
            acc
        }
    });
    println!("{}", floor);
}

pub fn _2(input: String) {
    let mut found = false;
    input.chars().enumerate().fold(0, |acc, (i, x)| {
        if acc == -1 && !found {
            println!("{}", i);
            found = true;
        }
        if x == '(' {
            acc + 1
        } else if x == ')' {
            acc - 1
        } else {
            acc
        }
    });
}
