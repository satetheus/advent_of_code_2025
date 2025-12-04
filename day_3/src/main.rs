use std::path::Path;


fn process_file(file: &Path) {
    todo!();
}


fn get_joltage(battery: String) -> i32 {
    let mut max_digit = '0';
    let mut max_digit_index = 0;
    let digits = battery.chars().collect::<Vec<_>>();

    for (i, digit) in digits.iter().enumerate().take(digits.len() - 1) {
        if *digit > max_digit {
            max_digit = *digit;
            max_digit_index = i;
        }
    }

    let mut next_max_digit = '0';
    for digit in digits.iter().skip(max_digit_index+1) {
        if *digit > next_max_digit {
            next_max_digit = *digit;
        }
    }

    (max_digit.to_string()+&next_max_digit.to_string())
        .parse::<i32>().expect("not a number")
}


fn main() {
    todo!();
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_joltage() {
        assert_eq!(get_joltage("987654321111111".to_string()), 98);
        assert_eq!(get_joltage("811111111111119".to_string()), 89);
        assert_eq!(get_joltage("234234234234278".to_string()), 78);
        assert_eq!(get_joltage("818181911112111".to_string()), 92);
    }
}
