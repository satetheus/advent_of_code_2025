use std::io::{BufReader,BufRead};
use std::fs::File;
use std::path::Path;


fn process_file(filename: impl AsRef<Path>) -> (Vec<[u128;2]>, Vec<u128>) {
    let file = File::open(filename).expect("file not found");
    let contents = BufReader::new(file);

    let lines: Vec<String> = contents.lines()
        .map(|n| n.expect("couldn't parse line"))
        .collect();
    let blank = lines.iter().position(|n| n.is_empty()).expect("couldn't find blank line");
    let ranges: Vec<[u128;2]> = lines[0..blank].iter()
        .map(|n| {
            let parts: Vec<u128> = n.split('-').filter_map(|i| Some(i.parse::<u128>().expect("not a number")))
            .collect::<Vec<u128>>();
            [parts[0],parts[1]]
        }).collect();
    let ids: Vec<u128> = lines[blank+1..].iter().map(|n| n.parse().expect("not a number")).collect();

    (ranges,ids)
}


fn count_fresh((ranges, ids): (Vec<[u128;2]>, Vec<u128>)) -> i32 {
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


fn get_fresh_ids(ranges: Vec<[u128;2]>) -> u128 {
    let (mut lows, mut highs): (Vec<u128>, Vec<u128>) = ranges.iter().map(|n| (n[0],n[1])).unzip();

    for (i, _low) in lows.clone().iter().enumerate() {
        for low_i in 0..lows.len()-1 {
            if low_i == i {
                continue
            }
            if lows[i] <= lows[low_i] && highs[low_i] <= highs[i] {
                lows[low_i] = 0;
                highs[low_i] = 0;
                continue
            }
            if lows[low_i] <= lows[i] && highs[i] <= highs[low_i] {
                lows[i] = 0;
                highs[i] = 0;
                continue
            }
            if lows[i] <= lows[low_i] && lows[low_i] <= highs[i] {
                lows[low_i] = 0;
                highs[i] = highs[low_i];
                highs[low_i] = 0;
                continue
            }
            if lows[i] <= highs[low_i] && highs[low_i] <= highs[i] {
                highs[low_i] = 0;
                lows[i] = lows[low_i];
                lows[low_i] = 0;
            }
        }
    }

    let totals: u128 = lows.into_iter().zip(highs).filter(|(l,h)| *l != 0 || *h != 0).map(|(l,h)| 1 + h - l).sum::<u128>();

    //ids.len()
    totals
}


fn main() {
    let input = process_file("day_5_input.txt");
    let count = count_fresh(input.clone());
    println!("{:?}", count);
    let all_fresh_ids_count = get_fresh_ids(input.0);
    println!("{:?}", all_fresh_ids_count);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_fresh() {
        assert_eq!(count_fresh((vec![[3,5],[10,14],[16,20],[12,18]],vec![1,5,8,11,17,32])), 3);
    }

    #[test]
    fn test_get_fresh_ids() {
        assert_eq!(get_fresh_ids(vec![[3,5],[10,14],[16,20],[12,18]]), 14);
        assert_eq!(get_fresh_ids(vec![[10,14],[16,20]]), 10);
        assert_eq!(get_fresh_ids(vec![[16,20],[10,14]]), 10);
        assert_eq!(get_fresh_ids(vec![[16,18],[16,20],[16,20]]), 5);
        assert_eq!(get_fresh_ids(vec![[16,18],[16,20],[16,20],[16,18]]), 5);
        assert_eq!(get_fresh_ids(vec![[9,21]]), 13);
        assert_eq!(get_fresh_ids(vec![[90876390158586,99371575473204]]), 8495185314619);
        assert_eq!(get_fresh_ids(vec![[10,20],[12,18]]), 11);
        assert_eq!(get_fresh_ids(vec![[12,18],[10,20]]), 11);
        assert_eq!(get_fresh_ids(vec![[14,20],[10,14],[10,14],[14,20]]), 11);
        assert_eq!(get_fresh_ids(vec![[114,120],[110,116],[110,116],[114,120]]), 11);
        assert_eq!(get_fresh_ids(vec![[16,120],[16,120],[16,120]]), 105);
        assert_eq!(get_fresh_ids(vec![[200,300],[100,101],[1,1],[2,2],[3,3],[1,3],[1,3],[2,2],[50,70],[10,10],[98,99],[99,99],[99,99],[99,100],[1,1],[2,1],[100,100],[100,100],[100,101],[200,300],[201,300],[202,300],[250,251],[98,99],[100,100],[100,101],[1,101]]), 202);
        assert_eq!(get_fresh_ids(vec![[10,10],[10,11]]), 2);
        assert_eq!(get_fresh_ids(vec![[10,20],[30,40],[50,60],[19,41],[39,51],[20,50],[1,100]]), 100);
        assert_eq!(get_fresh_ids(vec![[1,20],[5,9],[8,30]]), 30);
        assert_eq!(get_fresh_ids(vec![[100,200]]), 101);
        assert_eq!(get_fresh_ids(vec![[13,20],[10,15],[10,15],[13,20]]), 11);
        assert_eq!(get_fresh_ids(vec![[3,5],[10,14],[16,20],[12,18],[9,21]]), 16);
        assert_eq!(get_fresh_ids(vec![[29305409989804,29305409989804]]), 1);
        assert_eq!(get_fresh_ids(vec![[2,3],[5,6],[1,8]]), 8);
        assert_eq!(get_fresh_ids(vec![[1,10],[12,20],[9,11]]), 20);
        assert_eq!(get_fresh_ids(vec![[1,3],[3,5]]), 5);
        assert_eq!(get_fresh_ids(vec![[3,5],[10,14],[16,20],[12,18],[13,14],[13,13]]), 14);
    }
}

