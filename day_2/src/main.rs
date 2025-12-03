use itertools::Itertools;
use std::path::Path;
use std::fs;


fn main() {
    let ranges = process_file("day_2_input.txt");
    let mut total: i64 = 0;
    for range in ranges {
        total += find_invalid_ids(range).iter().sum::<i64>();
    }

    println!("{:?}", total);
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

        if check_id_by_chunk(i, chunk_size) {
            invalids.push(i);
        }
    }

    invalids
}


fn find_invalid_ids_part2(range: Vec<i64>) -> Vec<i64> {
    let min = *range.iter().min().expect("couldn't get min");
    let max = *range.iter().max().expect("couldn't get max");
    let mut invalids: Vec<i64> = vec![];

    for i in min..=max {
        let chunks = get_multiples(i.to_string().len());
        dbg!(&chunks);

        for chunk_size in chunks {
            if check_id_by_chunk(i, chunk_size) {
                invalids.push(i);
            }
        }
    }

    invalids.into_iter().unique().collect()
}


fn get_multiples(num: usize) -> Vec<usize> {
    let mut mults: Vec<usize> = vec![];

    for i in 1..=num/2 {
        if num % i == 0 {
            mults.push(i);
        }
    }
    mults
}


fn check_id_by_chunk(id: i64, chunk_size: usize) -> bool {
    let test: Vec<_> = id.to_string().chars().collect();
    let test2 = test.chunks(chunk_size).map(|n| n.iter().collect::<String>()).collect::<Vec<_>>();
    let deduped: Vec<_> = test2.iter().unique().collect();

    deduped.len() == 1
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_id_by_chunk() {
        assert!(check_id_by_chunk(11,1));
        assert!(!check_id_by_chunk(12,1));
        assert!(!check_id_by_chunk(1011,2));
        assert!(check_id_by_chunk(1010,2));
        assert!(!check_id_by_chunk(1010,1));
    }

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

    #[test]
    fn test_find_invalid_ids_part2() {
        assert_eq!(find_invalid_ids_part2(vec![11,22]), vec![11,22]);
        assert_eq!(find_invalid_ids_part2(vec![95,115]), vec![99,111]);
        assert_eq!(find_invalid_ids_part2(vec![998,1012]), vec![999,1010]);
        assert_eq!(find_invalid_ids_part2(vec![1188511880,1188511890]), vec![1188511885]);
        assert_eq!(find_invalid_ids_part2(vec![222220,222224]), vec![222222]);
        assert_eq!(find_invalid_ids_part2(vec![1698522,1698528]), vec![]);
        assert_eq!(find_invalid_ids_part2(vec![446443,446449]), vec![446446]);
        assert_eq!(find_invalid_ids_part2(vec![38593856,38593862]), vec![38593859]);
        assert_eq!(find_invalid_ids_part2(vec![2121212118,2121212124]), vec![2121212121]);

    }

    #[test]
    fn test_get_multiples() {
        assert_eq!(get_multiples(6), vec![1,2,3]);
        assert_eq!(get_multiples(5), vec![1]);
        assert_eq!(get_multiples(2), vec![1]);
    }
}
