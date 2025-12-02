use std::io::Read;
use std::path::Path;
use std::fs::File;


fn main() {
    // get maximum length of repeatable pattern
    // get limit from maximum in range
    // increment through all the possible repeats to validate
    // store the repeats as they come
    // sum them at the end (could also do rolling sum)
    process_file("day_2_input.txt");
}


fn process_file(filename: impl AsRef<Path>) -> Vec<String> {
    let mut file = File::open(filename).expect("file not found");
    let mut contents = String::new();
    let _ = file.read_to_string(&mut contents);

    dbg!(&contents);
    let lines: Vec<_> = contents.to_string().split(',').map(|n| n.to_string()).collect();

    dbg!(&lines);
    lines
}
