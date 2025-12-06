use std::io::{BufReader,BufRead};
use std::fs::File;
use std::path::Path;


fn process_file(filename: impl AsRef<Path>) -> (Vec<[i64;2]>, Vec<i64>) {
    let file = File::open(filename).expect("file not found");
    let contents = BufReader::new(file);

    let lines: Vec<String> = contents.lines()
        .map(|n| n.expect("couldn't parse line"))
        .collect();
    let blank = lines.iter().position(|n| n == "").expect("couldn't find blank line");
    let ranges: Vec<[i64;2]> = lines[0..blank].iter()
        .map(|n| {
            let parts: Vec<i64> = n.split('-').filter_map(|i| Some(i.parse::<i64>().expect("not a number")))
            .collect::<Vec<i64>>();
            [parts[0],parts[1]]
        }).collect();
    let ids: Vec<i64> = lines[blank+1..].iter().map(|n| n.parse().expect("not a number")).collect();

    (ranges,ids)
}


fn count_fresh((ranges, ids): (Vec<[i64;2]>, Vec<i64>)) -> i32 {
    let mut total = 0;
    for id in ids {
        for range in &ranges {
            if id >= range[0]  && id <= range[1] {
                total += 1;
                break
            }
        }
    }

    total
}


fn main() {
    let input = process_file("day_5_input.txt");
    let count = count_fresh(input);
    println!("{:?}", count);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_fresh() {
        assert_eq!(count_fresh((vec![[3,5],[10,14],[16,20],[12,18]],vec![1,5,8,11,17,32])), 3);
    }
}
