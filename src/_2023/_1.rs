pub fn _1(input: String) {
    let mut result = 0;
    for line in input.lines().into_iter() {
        let mut first_digit = -1;
        let mut last_digit = -1;
        for c in line.chars() {
            if let Ok(digit) = c.to_string().parse::<i32>() {
                if first_digit == -1 {
                    first_digit = digit;
                }
                last_digit = digit;
            }
        }
        result += first_digit * 10 + last_digit;
    }
    println!("{}", result)
}

pub fn _2(input: String) {
    
}
