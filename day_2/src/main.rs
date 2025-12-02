use itertools::Itertools;
use std::path::Path;
use std::fs;


fn main() {
    process_file("day_2_input.txt");
}


fn process_file(filename: impl AsRef<Path>) -> Vec<Vec<i64>> {
    let file = fs::read_to_string(filename).expect("file not found");
    let contents = file.strip_suffix("\n").expect("Couldn't strip newlines");

    // Not proud of this nested closure. Something is probably wrong here
    // but the output is correct.
    let lines: Vec<_> = contents.split(',').map(|n| n.to_string()).collect();
    let split_lines: Vec<Vec<_>> = lines.iter().map(|l|
        l.split('-').collect::<Vec<_>>().into_iter().map(|s|
            s.parse::<i64>().expect("error converting to number")
        ).collect()
    ).collect();

    split_lines
}


fn find_invalid_ids(range: Vec<i64>) -> Vec<i64> {
    let min = *range.iter().min().expect("couldn't get min");
    let max = *range.iter().max().expect("couldn't get max");
    let mut invalids: Vec<i64> = vec![];
    let mut chunk_size: usize;

    for i in min..=max {
        if i.to_string().len() % 2 == 0 {
            chunk_size = i.to_string().len()/2;
        } else {
            continue;
        }
        let test: Vec<_> = i.to_string().chars().collect();
        let test2 = test.chunks(chunk_size).map(|n| n.iter().collect::<String>()).collect::<Vec<_>>();
        let deduped: Vec<_> = test2.iter().unique().collect();

        if deduped.len() == 1 {
            invalids.push(i);
        }
    }

    invalids
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_invalid_ids() {
        assert_eq!(find_invalid_ids(vec![11,22]), vec![11,22]);
        assert_eq!(find_invalid_ids(vec![95,115]), vec![99]);
        assert_eq!(find_invalid_ids(vec![998,1012]), vec![1010]);
        assert_eq!(find_invalid_ids(vec![1188511880,1188511890]), vec![1188511885]);
        assert_eq!(find_invalid_ids(vec![222220,222224]), vec![222222]);
        assert_eq!(find_invalid_ids(vec![1698522,1698528]), vec![]);
        assert_eq!(find_invalid_ids(vec![446443,446449]), vec![446446]);
        assert_eq!(find_invalid_ids(vec![38593856,38593862]), vec![38593859]);
        assert_eq!(find_invalid_ids(vec![2121212118,2121212124]), vec![]);
    }
}
