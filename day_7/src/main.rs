use utils::{process_file,transpose};


fn main() {
    let input = process_file("day_7_input.txt");
    let part_1 = count_splits(run_laser(input.clone()));
    println!("{}", part_1);
    let part_2 = run_number_laser(input);
    println!("{}", part_2);
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


fn wonky_add(a: String, b:String) -> String {
    (a.parse::<i64>().unwrap_or(0) + b.parse::<i64>().unwrap_or(0)).to_string()
}


fn run_number_laser(input: Vec<String>) -> i64 {
    let mut grid: Vec<Vec<String>> = input.iter().map(|n| n.chars().map(|i| i.to_string()).collect()).collect();

    // rust really did not like what I was trying to do here with the mix types
    // in the 2d vec, so I had to do a lot of type conversions & clones to make
    // it work.
    for (i, line) in grid.clone().iter().enumerate() {
        if i == 0 { continue }
        for (j, item) in line.iter().enumerate() {
            if *item != "." { continue }
            grid[i][j] = "0".to_string();
            let above = grid[i-1][j].clone();
            let left = line.get((j as isize-1) as usize).unwrap_or(&".".to_string()).clone();
            let right = line.get((j as isize+1) as usize).unwrap_or(&".".to_string()).clone();

            if above == "S" { grid[i][j] = "1".to_string(); }

            if above.chars().next().expect("formats wrong").is_numeric() {
                grid[i][j] = wonky_add(grid[i][j].clone(),above);
            }

            if *left == "^".to_string() {
                grid[i][j] = wonky_add(grid[i][j].clone(),grid[i-1][j-1].clone());
            }

            if *right == "^".to_string() {
                grid[i][j] = wonky_add(grid[i][j].clone(),grid[i-1][j+1].clone());
            }
        }
    }

    let final_numbers = grid.pop().expect("probably a parsing error");
    final_numbers.iter().map(|n| n.parse::<i64>().unwrap_or(0)).sum::<i64>()
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
    fn test_run_number_laser() {
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

        assert_eq!(run_number_laser(ex_input), 40);
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
