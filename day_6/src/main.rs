use utils::{process_file, transpose, split_str};


fn main() {
    let input = process_file("day_6_input.txt");
    let part_1_output = do_operations(input.clone());
    println!("{:?}", part_1_output);
    let part_2_output = do_wonky_operations(input);
    println!("{:?}", part_2_output);
}


fn do_operations(input: Vec<String>) -> i64 {
    let parsed_input = input.iter().map(|n| split_str!(n)).collect();
    let pivot = transpose(parsed_input);
    let mut total = 0;

    for mut set in pivot {
        let operator = set.pop().expect("no operator found");
        let numbers: Vec<i64> = set.iter().map(|n| n.parse().expect("not a number")).collect();
        if operator == "+" {
            total += numbers.iter().sum::<i64>();
        } else {
            let mut product = 1;
            for i in numbers {
                product *= i;
            }
            total += product;
        }
    }

    total
}


fn do_wonky_operations(input: Vec<String>) -> i64 {
    let parsed_input: Vec<Vec<char>> = input.iter().map(|n| n.chars().collect()).collect();
    let mut total = 0;
    let pivot = transpose(parsed_input);
    let mut operators: Vec<char> = vec![];
    let mut numbers: Vec<i64> = vec![];
    for mut set in pivot {
        operators.push(set.pop().expect("no operator found"));
        numbers.push(set.iter().filter(|n| **n != ' ').collect::<String>().parse().unwrap_or(0));
    }

    let mut action = ' ';
    let mut op_total = 0;
    for (i, operator) in operators.iter().enumerate() {
        if numbers[i] == 0 { continue }
        if *operator != ' ' {
            action = *operator;
            total += op_total;
            if *operator == '+' { op_total = 0; }
            else { op_total = 1; }
        }

        if action == '+' {
            op_total += numbers[i];
        } else {
            op_total *= numbers[i];
        }

        if i == operators.len()-1 { total += op_total; }
    }

    total
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_do_operations() {
        let input: Vec<String> = vec!["123 328 51 64".to_string(),"45 64 387 23".to_string(),"6 98 215 314".to_string(),"* + * +".to_string()];
        assert_eq!(do_operations(input), 4277556);
    }

    #[test]
    fn test_do_wonky_operations() {
        let input: Vec<String> = vec!["123 328  51 64 ".to_string()," 45 64  387 23 ".to_string(),"  6 98  215 314".to_string(),"*   +   *   +  ".to_string()];
        assert_eq!(do_wonky_operations(input), 3263827);
    }
}
