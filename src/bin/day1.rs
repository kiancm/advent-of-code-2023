use std::{fs, path::PathBuf};

fn main() -> std::io::Result<()> {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("data/day1/input.txt");

    let input = fs::read_to_string(path)?;

    let sum: u32 = input.lines()
    .filter_map(|line| {
        let nums: Vec<char> = line.chars().filter(|char| char.is_numeric()).collect();
        match nums.as_slice() {
            [only] => concat_digits(only.to_owned(), only.to_owned()),
            [first, .., last] => concat_digits(first.to_owned(), last.to_owned()),
            _ => None
        }
    })
    .sum();

    println!("Sum: {:?}", sum);

    Ok(())
}

fn concat_digits(first: char, last: char) -> Option<u32> {
    let mut str = first.to_string();
    str.push(last);
    str.parse().ok()
}
