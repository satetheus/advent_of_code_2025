use std::collections::HashSet;
use std::path::Path;
use std::fs;


fn main() {
    let ranges = process_file("day_2_input.txt");
    let mut total_p1: i64 = 0;
    let mut total_p2: i64 = 0;
    for range in &ranges {
        total_p1 += find_invalid_ids(range).0.iter().sum::<i64>();
        total_p2 += find_invalid_ids(range).1.iter().sum::<i64>();
    }
    println!("Part 1: {:?}", total_p1);
    println!("Part 2: {:?}", total_p2);
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


fn find_invalid_ids(range: &[i64]) -> (Vec<i64>,HashSet<i64>) {
    let mut invalids_p1: Vec<i64> = vec![];
    let mut invalids_p2: HashSet<i64> = HashSet::new();

    for i in range[0]..=range[1] {
        let id = i.to_string();
        let chunks = get_multiples(id.len());
        for chunk_size in chunks {
            if check_id_by_chunk(&id, chunk_size) {
                if id.len() % 2 == 0 && chunk_size == id.len()/2 {
                    invalids_p1.push(i);
                }

                invalids_p2.insert(i);
            }
        }

    }

    (invalids_p1, invalids_p2)
}


fn get_multiples(num: usize) -> Vec<usize> {
    (1..=num/2).filter(|i| num % i == 0).collect()
}


fn check_id_by_chunk(id: &str, chunk_size: usize) -> bool {
    let mut test = id.as_bytes().chunks(chunk_size);
    let chunk_1 = test.next().expect("couldn't get first chunk of string");

    test.all(|n| n == chunk_1)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_id_by_chunk() {
        assert!(check_id_by_chunk("11",1));
        assert!(!check_id_by_chunk("12",1));
        assert!(!check_id_by_chunk("1011",2));
        assert!(check_id_by_chunk("1010",2));
        assert!(!check_id_by_chunk("1010",1));
    }

    #[test]
    fn test_find_invalid_ids() {
        assert_eq!(find_invalid_ids(&[11,22]), (vec![11,22],HashSet::from([11,22])));
        assert_eq!(find_invalid_ids(&[95,115]), (vec![99],HashSet::from([99,111])));
        assert_eq!(find_invalid_ids(&[998,1012]), (vec![1010],HashSet::from([999,1010])));
        assert_eq!(find_invalid_ids(&[1188511880,1188511890]), (vec![1188511885],HashSet::from([1188511885])));
        assert_eq!(find_invalid_ids(&[222220,222224]), (vec![222222],HashSet::from([222222])));
        assert_eq!(find_invalid_ids(&[1698522,1698528]), (vec![],HashSet::from([])));
        assert_eq!(find_invalid_ids(&[446443,446449]), (vec![446446],HashSet::from([446446])));
        assert_eq!(find_invalid_ids(&[38593856,38593862]), (vec![38593859],HashSet::from([38593859])));
        assert_eq!(find_invalid_ids(&[2121212118,2121212124]), (vec![],HashSet::from([2121212121])));
    }

    #[test]
    fn test_get_multiples() {
        assert_eq!(get_multiples(6), vec![1,2,3]);
        assert_eq!(get_multiples(5), vec![1]);
        assert_eq!(get_multiples(2), vec![1]);
    }
}
