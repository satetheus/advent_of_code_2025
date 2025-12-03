use std::path::Path;


fn process_file(file: &Path) {
    todo!();
}


fn get_joltage(battery: i64) -> i64 {
    todo!();
}


fn main() {
    todo!();
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_joltage() {
        assert_eq!(get_joltage(987654321111111), 98);
        assert_eq!(get_joltage(811111111111119), 89);
        assert_eq!(get_joltage(234234234234278), 78);
        assert_eq!(get_joltage(818181911112111), 92);
    }
}
