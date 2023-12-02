use std::{
    collections::{HashMap, HashSet},
    fs,
    path::PathBuf,
};

fn main() -> std::io::Result<()> {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("data/day1/input.txt");

    let input = fs::read_to_string(path)?;

    let digit_spellings = HashMap::from([
        ("one", "1".to_string()),
        ("two", "2".to_string()),
        ("three", "3".to_string()),
        ("four", "4".to_string()),
        ("five", "5".to_string()),
        ("six", "6".to_string()),
        ("seven", "7".to_string()),
        ("eight", "8".to_string()),
        ("nine", "9".to_string()),
    ]);

    let substrings: HashSet<String> = digit_spellings
        .keys()
        .flat_map(|s| {
            let mut substrings = Vec::new();
            let mut substring = String::new();
            for char in s.chars() {
                substring.push(char);
                substrings.push(substring.clone());
            }
            substrings
        })
        .collect();

    let sum: u32 = input
        .lines()
        .filter_map(|line| {
            let mut digits: Vec<String> = Vec::new();
            let mut current = String::new();
            for char in line.chars() {
                if let Some(v) = digit_spellings.get(current.as_str()) {
                    digits.push(v.to_owned());
                    current = String::new();
                }

                if !substrings.contains(&current) {
                    current = String::new();
                }

                if char.is_numeric() {
                    digits.push(char.to_string());
                } else {
                    current.push(char);
                }
            }
            if let Some(v) = digit_spellings.get(current.as_str()) {
                digits.push(v.to_owned());
            }
            match digits.as_slice() {
                [only] => concat_digits(only, only),
                [first, .., last] => concat_digits(first, last),
                _ => None,
            }
        })
        .sum();

    println!("Sum: {:?}", sum);

    Ok(())
}

fn concat_digits(first: &String, last: &String) -> Option<u32> {
    let mut str = String::new();
    str.push_str(first);
    str.push_str(last);

    str.parse().ok()
}
