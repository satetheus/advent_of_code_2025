use utils::process_file;

fn main() {
    let points = strings_to_points(process_file("day_9_input.txt"));
    let part_1 = get_largest_square(points);
    println!("{}", part_1);
}

fn strings_to_points(input: Vec<String>) -> Vec<[i64; 2]> {
    input
        .iter()
        .map(|n| {
            n.split(",")
                .map(|i| i.parse().expect("not a number"))
                .collect::<Vec<i64>>()
                .try_into()
                .expect("incorrect length")
        })
        .collect()
}

fn get_largest_square(points: Vec<[i64; 2]>) -> i64 {
    let mut sqrs: Vec<i64> = vec![];

    for i in 0..points.len().pow(2) {
        let a = i / points.len();
        let b = i % points.len();

        sqrs.push(
            ((points[a][0] - points[b][0]).abs() + 1) * ((points[a][1] - points[b][1]).abs() + 1),
        );
    }

    *sqrs.iter().max().expect("couldn't get max")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_largest_square() {
        assert_eq!(
            get_largest_square(vec![
                [7, 1],
                [11, 1],
                [11, 7],
                [9, 7],
                [9, 5],
                [2, 5],
                [2, 3],
                [7, 3]
            ]),
            50
        );
    }
}
