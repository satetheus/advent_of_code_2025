use std::io::{BufReader,BufRead};
use std::fs::File;
use std::path::Path;


fn main() {
    let input = process_file("day_6_input.txt");
    println!("{:?}", input);
}


fn process_file(filename: impl AsRef<Path>) -> Vec<Vec<String>> {
    let file = File::open(filename).expect("file not found");
    let contents = BufReader::new(file);

    let lines: Vec<String> = contents.lines()
        .map(|n| n.expect("couldn't parse line"))
        .collect();

    lines.iter().map(|n| n.split_whitespace().map(String::from).collect()).collect()
}


fn do_operations(input: Vec<Vec<String>>) -> i64 {
    todo!();
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_do_operations() {
        let input: Vec<String> = vec!["123 328 51 64".to_string(),"45 64 387 23".to_string(),"6 98 215 314".to_string(),"* + * +".to_string()];
        let processed_input = input.iter().map(|n| n.split_whitespace().map(String::from).collect()).collect();
        assert_eq!(do_operations(processed_input), 4277556);
    }
}
