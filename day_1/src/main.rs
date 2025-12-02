use std::fs::File;
use std::io::{self, BufRead};

fn process_file(file: io::Lines<io::BufReader<File>>) -> (i32, i32) {
    let mut zero_counter = 0;
    let mut pass_zero_counter = 0;
    let mut current_position: i32 = 50;

    for line in file {
        let line = match line {
            Ok(line) => line,
            Err(_) => return (0, 0),
        };

        let mut change_by = line[1..].parse::<i32>().expect("should be number");
        if line.starts_with("L") {
            change_by *= -1;
        }
        let old_position = current_position;
        current_position += change_by;

        if current_position != current_position.rem_euclid(100) || current_position == 0 {
            let mut increment = current_position / 100;
            if increment <= 0 && old_position != 0 {
                increment -= 1;
            }
            if !(old_position == 0 && current_position.abs() < 100) {
                pass_zero_counter += 1.max(increment.abs());
            }
            current_position = current_position.rem_euclid(100);
        }

        if current_position == 0 {
            zero_counter += 1;
        }
    }

    (zero_counter, pass_zero_counter)
}

fn main() -> std::io::Result<()> {
    let file = io::BufReader::new(File::open("day_1_input.txt")?).lines();
    let results = process_file(file);

    println!("{:?}", results);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_file() -> Result<(), Box<dyn std::error::Error>> {
        let file = io::BufReader::new(File::open("test_data/test_data_1.txt")?).lines();
        let result = process_file(file);
        assert_eq!(result, (3, 6));

        let file = io::BufReader::new(File::open("test_data/test_data_2.txt")?).lines();
        let result = process_file(file);
        assert_eq!(result, (0, 24));

        let file = io::BufReader::new(File::open("test_data/test_data_3.txt")?).lines();
        let result = process_file(file);
        assert_eq!(result, (1, 1));

        let file = io::BufReader::new(File::open("test_data/test_data_4.txt")?).lines();
        let result = process_file(file);
        assert_eq!(result, (1, 1));

        let file = io::BufReader::new(File::open("test_data/test_data_5.txt")?).lines();
        let result = process_file(file);
        assert_eq!(result, (1, 1));

        let file = io::BufReader::new(File::open("test_data/test_data_6.txt")?).lines();
        let result = process_file(file);
        assert_eq!(result, (1, 1));

        let file = io::BufReader::new(File::open("test_data/test_data_7.txt")?).lines();
        let result = process_file(file);
        assert_eq!(result, (1, 2));

        let file = io::BufReader::new(File::open("test_data/test_data_8.txt")?).lines();
        let result = process_file(file);
        assert_eq!(result, (1, 2));

        let file = io::BufReader::new(File::open("test_data/test_data_9.txt")?).lines();
        let result = process_file(file);
        assert_eq!(result, (1, 2));

        let file = io::BufReader::new(File::open("test_data/test_data_10.txt")?).lines();
        let result = process_file(file);
        assert_eq!(result, (1, 2));

        let file = io::BufReader::new(File::open("test_data/test_data_11.txt")?).lines();
        let result = process_file(file);
        assert_eq!(result, (0, 10));

        let file = io::BufReader::new(File::open("test_data/test_data_12.txt")?).lines();
        let result = process_file(file);
        assert_eq!(result, (0, 2));

        let file = io::BufReader::new(File::open("test_data/test_data_13.txt")?).lines();
        let result = process_file(file);
        assert_eq!(result, (1, 3));

        let file = io::BufReader::new(File::open("test_data/test_data_14.txt")?).lines();
        let result = process_file(file);
        assert_eq!(result, (0, 3));

        let file = io::BufReader::new(File::open("test_data/test_data_15.txt")?).lines();
        let result = process_file(file);
        assert_eq!(result, (2, 3));
        Ok(())
    }
}
