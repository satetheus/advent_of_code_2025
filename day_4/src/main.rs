use std::cmp::min;
use std::io::{BufReader,BufRead};
use std::fs::File;
use std::path::Path;


fn main() {
    let file = process_file("day_4_input.txt");
    let mut previous_state = file.clone();

    let (count_part1, _) = count_from_state(file);
    println!("{}", count_part1);

    let mut total = 0;
    loop {
        let (count, new_state) = count_from_state(previous_state.clone());
        if new_state == previous_state {
            break
        }
        total += count;
        previous_state = new_state;
    }
    println!("{}", total);
}


fn process_file(filename: impl AsRef<Path>) -> Vec<Vec<char>> {
    let file = File::open(filename).expect("file not found");
    let contents = BufReader::new(file);

    contents.lines()
        .map(|n| n.expect("couldn't parse line").chars().collect())
        .collect()
}


fn safe_sub(one: usize, two: usize) -> usize {
    one.checked_sub(two).unwrap_or(0)
}


fn safe_add(one: usize, two: usize, max: usize) -> usize {
    min(one+two,max)
}


fn count_from_state(input: Vec<Vec<char>>) -> (i32,Vec<Vec<char>>) {
    let mut total: i32 = 0;
    let mut output = input.clone();
    for (i, row) in input.iter().enumerate() {
        for (j, item) in row.iter().enumerate() {
            if *item == '@' {
                let mut window: Vec<char> = vec![];
                if (i as i32 - 1) >= 0 {
                    window.append(&mut input[safe_sub(i,1)][safe_sub(j,1)..=safe_add(j,1,row.len()-1)].to_vec());
                }
                window.append(&mut input[i][safe_sub(j,1)..=safe_add(j,1,row.len()-1)].to_vec());
                if (i + 1) < input.len() {
                    window.append(&mut input[safe_add(i,1,input.len()-1)][safe_sub(j,1)..=safe_add(j,1,row.len()-1)].to_vec());
                }

                if window.into_iter().filter(|n| *n == '@').collect::<Vec<char>>().len() < 5 {
                    output[i][j] = 'x';
                    total += 1;
                }
            }
        }
    }

    (total, output)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_from_state() {
        let input: Vec<Vec<char>> = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.".split("\n").map(|n| n.chars().collect()).collect();
        assert_eq!(count_from_state(input), (13,
                vec![
                vec!['.','.','x','x','.','x','x','@','x','.'],
                vec!['x','@','@','.','@','.','@','.','@','@'],
                vec!['@','@','@','@','@','.','x','.','@','@'],
                vec!['@','.','@','@','@','@','.','.','@','.'],
                vec!['x','@','.','@','@','@','@','.','@','x'],
                vec!['.','@','@','@','@','@','@','@','.','@'],
                vec!['.','@','.','@','.','@','.','@','@','@'],
                vec!['x','.','@','@','@','.','@','@','@','@'],
                vec!['.','@','@','@','@','@','@','@','@','.'],
                vec!['x','.','x','.','@','@','@','.','x','.'],
                ]
            )
        );
    }
}
