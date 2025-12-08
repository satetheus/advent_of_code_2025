use std::io::{BufReader,BufRead};
use std::fs::File;
use std::path::Path;


fn main() {
    let input = process_file("day_7_input.txt");
    let part_1 = count_splits(run_laser(input));
    println!("{}", part_1);
}


fn process_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("file not found");
    let contents = BufReader::new(file);

    contents.lines()
        .map(|n| n.expect("couldn't parse line"))
        .collect::<Vec<String>>()
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


fn run_laser(input: Vec<String>) -> Vec<Vec<char>> {
    let mut grid: Vec<Vec<char>> = input.iter().map(|n| n.chars().collect()).collect();

    for (i, line) in grid.clone().iter().enumerate() {
        if i == 0 { continue }
        for (j, item) in line.iter().enumerate() {
            if *item != '.' { continue }
            let above = &grid[i-1][j];
            let left = *line.get((j as isize-1) as usize).unwrap_or(&'.');
            let right = *line.get((j as isize+1) as usize).unwrap_or(&'.');

            if left == '.' && right == '.' && !['S','|'].contains(above) { continue }

            grid[i][j] = '|'
        }
    }

    grid
}


fn count_splits(input: Vec<Vec<char>>) -> usize {
    let pivot = transpose(input);
    let joined: Vec<String> = pivot.iter().map(|n| n.into_iter().collect()).collect();

    joined.iter().map(|n| n.matches("|^").count()).sum()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_laser() {
        let ex_input = vec![".......S.......".to_string(),
        "...............".to_string(),
        ".......^.......".to_string(),
        "...............".to_string(),
        "......^.^......".to_string(),
        "...............".to_string(),
        ".....^.^.^.....".to_string(),
        "...............".to_string(),
        "....^.^...^....".to_string(),
        "...............".to_string(),
        "...^.^...^.^...".to_string(),
        "...............".to_string(),
        "..^...^.....^..".to_string(),
        "...............".to_string(),
        ".^.^.^.^.^...^.".to_string(),
        "...............".to_string()];

        assert_eq!(run_laser(ex_input),
        vec![vec!['.','.','.','.','.','.','.','S','.','.','.','.','.','.','.'],
        vec!['.','.','.','.','.','.','.','|','.','.','.','.','.','.','.'],
        vec!['.','.','.','.','.','.','|','^','|','.','.','.','.','.','.'],
        vec!['.','.','.','.','.','.','|','.','|','.','.','.','.','.','.'],
        vec!['.','.','.','.','.','|','^','|','^','|','.','.','.','.','.'],
        vec!['.','.','.','.','.','|','.','|','.','|','.','.','.','.','.'],
        vec!['.','.','.','.','|','^','|','^','|','^','|','.','.','.','.'],
        vec!['.','.','.','.','|','.','|','.','|','.','|','.','.','.','.'],
        vec!['.','.','.','|','^','|','^','|','|','|','^','|','.','.','.'],
        vec!['.','.','.','|','.','|','.','|','|','|','.','|','.','.','.'],
        vec!['.','.','|','^','|','^','|','|','|','^','|','^','|','.','.'],
        vec!['.','.','|','.','|','.','|','|','|','.','|','.','|','.','.'],
        vec!['.','|','^','|','|','|','^','|','|','.','|','|','^','|','.'],
        vec!['.','|','.','|','|','|','.','|','|','.','|','|','.','|','.'],
        vec!['|','^','|','^','|','^','|','^','|','^','|','|','|','^','|'],
        vec!['|','.','|','.','|','.','|','.','|','.','|','|','|','.','|']]);
    }

    #[test]
    fn test_count_splits() {
        let ex_input = vec![".......S.......".to_string(),
        "...............".to_string(),
        ".......^.......".to_string(),
        "...............".to_string(),
        "......^.^......".to_string(),
        "...............".to_string(),
        ".....^.^.^.....".to_string(),
        "...............".to_string(),
        "....^.^...^....".to_string(),
        "...............".to_string(),
        "...^.^...^.^...".to_string(),
        "...............".to_string(),
        "..^...^.....^..".to_string(),
        "...............".to_string(),
        ".^.^.^.^.^...^.".to_string(),
        "...............".to_string()];

        assert_eq!(count_splits(run_laser(ex_input)), 21);
    }
}
