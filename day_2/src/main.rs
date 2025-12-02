use std::path::Path;
use std::fs;


fn main() {
    // get maximum length of repeatable pattern
    // get limit from maximum in range
    // increment through all the possible repeats to validate
    // store the repeats as they come
    // sum them at the end (could also do rolling sum)
    process_file("day_2_input.txt");
}


fn process_file(filename: impl AsRef<Path>) -> Vec<Vec<String>> {
    let file = fs::read_to_string(filename).expect("file not found");
    let contents = file.strip_suffix("\n").expect("Couldn't strip newlines");

    // Not proud of this nested closure. Something is probably wrong here
    // but the output is correct.
    let lines: Vec<_> = contents.split(',').map(|n| n.to_string()).collect();
    let split_lines: Vec<Vec<_>> = lines.iter().map(|l|
        l.split('-').collect::<Vec<_>>().into_iter().map(|s|
            s.to_string()
        ).collect()
    ).collect();

    split_lines
}


fn find_invalid_ids() {
    todo!();
}
