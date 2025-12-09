use itertools::Itertools;
use utils::process_file;

fn main() {
    let input = string_to_points(process_file("day_8_input.txt"));
    let _output = get_distance_list(input);
    println!("This ran!");
}

fn string_to_points(input: Vec<String>) -> Vec<[i64; 3]> {
    input
        .iter()
        .map(|n| {
            n.split(',')
                // Clippy says that this can just be map. It cannot.
                .filter_map(|i| Some(i.parse::<i64>().expect("not a number")))
                .collect::<Vec<i64>>()
                .try_into()
                .expect("getting ridiculous")
        })
        .collect()
}

fn euc_sqr_dist(a: [i64; 3], b: [i64; 3]) -> i64 {
    let dist = (b[0] - a[0]).pow(2) + (b[1] - a[1]).pow(2) + (b[2] - a[2]).pow(2);
    if dist == 0 {
        return i64::MAX;
    }
    dist
}

fn get_distance_list(input: Vec<[i64; 3]>) -> Vec<Vec<[i64; 3]>> {
    let mut dists: Vec<Vec<[i64; 3]>> = vec![];

    for point in &input {
        let min = input
            .iter()
            .min_by_key(|n| euc_sqr_dist(**n, *point))
            .expect("something's wrong");
        dists.push(vec![*point, *min]);
    }

    for item in &mut dists {
        item.sort();
    }

    dists.into_iter().unique().collect::<Vec<Vec<[i64; 3]>>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_euc_sqr_dist() {
        assert_eq!(euc_sqr_dist([162, 817, 812], [431, 825, 988]), 103401);
    }

    #[test]
    fn test_get_distance_list() {
        assert_eq!(
            get_distance_list(vec![
                [162, 817, 812],
                [57, 618, 57],
                [906, 360, 560],
                [592, 479, 940],
                [352, 342, 300],
                [466, 668, 158],
                [542, 29, 236],
                [431, 825, 988],
                [739, 650, 466],
                [52, 470, 668],
                [216, 146, 977],
                [819, 987, 18],
                [117, 168, 530],
                [805, 96, 715],
                [346, 949, 466],
                [970, 615, 88],
                [941, 993, 340],
                [862, 61, 35],
                [984, 92, 344],
                [425, 690, 689]
            ]),
            vec![
                vec![[162, 817, 812], [425, 690, 689]],
                vec![[57, 618, 57], [466, 668, 158]],
                vec![[805, 96, 715], [906, 360, 560]],
                vec![[425, 690, 689], [592, 479, 940]],
                vec![[352, 342, 300], [542, 29, 236]],
                vec![[352, 342, 300], [466, 668, 158]],
                vec![[162, 817, 812], [431, 825, 988]],
                vec![[739, 650, 466], [906, 360, 560]],
                vec![[52, 470, 668], [117, 168, 530]],
                vec![[117, 168, 530], [216, 146, 977]],
                vec![[819, 987, 18], [941, 993, 340]],
                vec![[346, 949, 466], [425, 690, 689]],
                vec![[819, 987, 18], [970, 615, 88]],
                vec![[862, 61, 35], [984, 92, 344]]
            ]
        );
    }
}
