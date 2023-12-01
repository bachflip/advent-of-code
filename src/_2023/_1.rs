use std::collections::BTreeMap;

pub fn _1(input: String) {
    let mut result = 0;
    for line in input.lines() {
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
    let digits = BTreeMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);
    
    let all_spelled_out = digits.keys().map(|s| s.to_string()).chain(digits.values().map(|d| d.to_string())).collect::<Vec<String>>();
    
    let mut result = 0;
    for line in input.lines() {
        let mut first_idx = line.len() - 1;
        let mut last_idx = 0;

        let mut first_digit = "".to_string();
        let mut last_digit = "".to_string();
            for d in &all_spelled_out {
            if let Some(left_find) = line.find(d) {
                if left_find <= first_idx {
                    first_idx = left_find; 
                    first_digit = d.to_string();
                }
            }

            if let Some(right_find) = line.rfind(d) {
                if right_find >= last_idx  {
                    last_idx = right_find; 
                    last_digit = d.to_string();
                }
            }
        }
        let mut first = 0;
        let mut last = 0;
        if let Ok(first_digit) = first_digit.parse::<i32>() {
            first = first_digit;
        } else {
            first = *digits.get(&first_digit.as_str()).unwrap();
        }
        if let Ok(last_digit) = last_digit.parse::<i32>() {
            last = last_digit;
        } else {
            last = *digits.get(&last_digit.as_str()).unwrap();
        }

        result += first * 10 + last;
    }

    println!("{}", result);
}
