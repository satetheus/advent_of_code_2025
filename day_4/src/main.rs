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


fn count_from_state(input: Vec<Vec<char>>) -> (usize,Vec<Vec<char>>) {
    let window_ind: [[isize;2];8] = [[-1,-1],[-1,0],[-1,1],[0,-1],[0,1],[1,-1],[1,0],[1,1]];
    let mut total: usize = 0;
    let mut output = vec![vec![' ';input[0].len()];input.len()];
    for (i, row) in input.iter().enumerate() {
        for (j, item) in row.iter().enumerate() {
            let mut window = vec![];
            if *item == '@' {
                for index in window_ind {
                    if i as isize + index[0] < 0 || i as isize + index[0] > input.len() as isize-1 {
                        continue
                    }
                    if j as isize + index[1] < 0 || j as isize + index[1] > row.len() as isize-1 {
                        continue
                    }
                    window.push(input[(i as isize+index[0]) as usize][(j as isize+index[1]) as usize]);
                }
                if window.iter().filter(|n| **n == '@').count() < 4 {
                    total += 1;
                    output[i][j] = 'x';
                    continue
                }
            }
            output[i][j] = input[i][j];
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
