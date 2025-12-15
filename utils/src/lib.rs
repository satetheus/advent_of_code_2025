// todo! these imports should be setup so they only are brought in if
// process_file is.
use std::io::{BufReader,BufRead};
use std::fs::File;
use std::path::Path;


pub fn process_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("file not found");
    let contents = BufReader::new(file);

    contents.lines()
        .map(|n| n.expect("couldn't parse line"))
        .collect::<Vec<String>>()
}


pub fn transpose<T>(input: Vec<Vec<T>>) -> Vec<Vec<T>> {
    let len = input[0].len();
    let mut iters: Vec<_> = input.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters.iter_mut().map(|n| n.next().expect("couldn't parse")).collect::<Vec<T>>()
        })
    .collect()
}


#[macro_export]
macro_rules! split_str {
    ($input: expr) => { $input.split_whitespace().map(String::from).collect() };
    // todo! implement a way to have unordered optional parameters
    ($input: expr, $split_val:expr) => {
        $input.split($split_val).map(|n| n.parse().expect("could not convert")).collect()
    };
    ($input: expr, $split_val:expr, $drop: expr) => {
        $input.split($split_val).map(|n| n.replace($drop, "").parse().expect("could not convert")).collect()
    }
}
