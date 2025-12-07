use std::io::{BufReader,BufRead};
use std::fs::File;
use std::path::Path;


fn main() {
    let input = process_file("day_6_input.txt");
    let pivot = transpose(input);
    let part_1_output = do_operations(pivot);
    println!("{:?}", part_1_output);
}


fn process_file(filename: impl AsRef<Path>) -> Vec<Vec<String>> {
    let file = File::open(filename).expect("file not found");
    let contents = BufReader::new(file);

    let lines: Vec<String> = contents.lines()
        .map(|n| n.expect("couldn't parse line"))
        .collect();

    lines.iter().map(|n| n.split_whitespace().map(String::from).collect()).collect()
}


fn transpose<T>(input: Vec<Vec<T>>) -> Vec<Vec<T>> {
    let len = input[0].len();
    let mut iters: Vec<_> = input.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters.iter_mut().map(|n| n.next().expect("couldn't parse")).collect::<Vec<T>>()
        })
    .collect()
}


fn do_operations(input: Vec<Vec<String>>) -> i64 {
    let mut total = 0;

    for mut set in input {
        let operator = set.pop().expect("no operator found");
        let numbers: Vec<i64> = set.iter().map(|n| n.parse().expect("not a number")).collect();
        if operator == "+" {
            total += numbers.iter().sum::<i64>();
        } else {
            let mut product = 1;
            for i in numbers {
                product *= i;
            }
            total += product;
        }
    }

    total
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_do_operations() {
        let input: Vec<String> = vec!["123 328 51 64".to_string(),"45 64 387 23".to_string(),"6 98 215 314".to_string(),"* + * +".to_string()];
        let processed_input = input.iter().map(|n| n.split_whitespace().map(String::from).collect()).collect();
        let pivoted_input = transpose(processed_input);
        assert_eq!(do_operations(pivoted_input), 4277556);
    }
}
