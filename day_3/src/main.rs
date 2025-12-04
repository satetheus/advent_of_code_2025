use std::path::Path;
use std::fs::File;
use std::io::{BufReader,BufRead};


fn process_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("file not found");
    let contents = BufReader::new(file);

    contents.lines()
        .map(|n| n.expect("couldn't parse line"))
        .collect()
}


fn get_joltage(battery: String, count: usize) -> i64 {
    let mut total_num = "".to_string();
    let digits = battery.chars().collect::<Vec<_>>();
    let mut previous_max_index = 0;

    for position in 0..count {
        let mut max_digit = '0';
        let mut max_digit_index = 0;
        for (i, digit) in digits.iter().enumerate() {
            if i <= previous_max_index && !(position == 0 && i == 0) { continue };
            if i > digits.len()-(count-position) { continue };
            if *digit > max_digit {
                max_digit = *digit;
                max_digit_index = i;
            }
        }
        previous_max_index = max_digit_index;
        total_num.push(max_digit);
    }

    total_num.parse::<i64>().expect("not a number")
}


fn main() {
    let input = process_file("day_3_input.txt");
    let mut total_part1 = 0;
    let mut total_part2 = 0;

    for line in input.clone() {
        total_part1 += get_joltage(line, 2);
    }

    for line in input {
        total_part2 += get_joltage(line, 12);
    }

    println!("{}", total_part1);
    println!("{}", total_part2);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_joltage() {
        assert_eq!(get_joltage("987654321111111".to_string(),2), 98);
        assert_eq!(get_joltage("811111111111119".to_string(),2), 89);
        assert_eq!(get_joltage("234234234234278".to_string(),2), 78);
        assert_eq!(get_joltage("818181911112111".to_string(),2), 92);
    }

    #[test]
    fn test_get_joltage_part2() {
        assert_eq!(get_joltage("987654321111111".to_string(),12), 987654321111);
        assert_eq!(get_joltage("811111111111119".to_string(),12), 811111111119);
        assert_eq!(get_joltage("234234234234278".to_string(),12), 434234234278);
        assert_eq!(get_joltage("818181911112111".to_string(),12), 888911112111);
    }
}
