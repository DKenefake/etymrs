#![crate_name = "etymrs"]

mod core;
mod stats;
mod word;

pub use core::*;
pub use stats::*;
pub use word::*;

#[cfg(test)]
mod tests {

    use crate::core::WordData;
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    fn test_read_json() {
        let word_data = WordData::generate_from_json("data//words.json");

        for v in word_data
            .get_value(&String::from("drink"), &String::from("English"))
            .unwrap()
        {
            println!("{}", v)
        }
    }

    #[test]
    fn test_analysis() {
        let word_data = WordData::generate_from_json("data//words.json");
        let s = |x: &str| String::from(x);

        let input = "You can feel that can't you It has an aura of jealousy"
            .split(" ")
            .map(|x| s(x))
            .collect();

        let a = word_data.simple_analysis(&input, &s("English"));

        for (key, value) in a.0 {
            println!("{} : {}", key, value);
        }
    }

    #[test]
    fn test_performance() {
        // create the word_data struct from the json file
        let word_data = WordData::generate_from_json("data//words.json");

        // read in a sample text
        let data = fs::read_to_string("data//book.txt").expect("Unable to read file");

        // tokenize and run
        let input = data.split(" ").map(|x| String::from(x)).collect();

        // get an initial time
        let start = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();

        // run the analysis
        let (results, not_counted) = word_data.simple_analysis(&input, &String::from("English"));

        // get an end time
        let end = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();

        println!(
            "Time to solve is {} ms with {} ns per word",
            (end - start) / 1_000_000u128,
            (end - start) / (input.len() as u128)
        );
    }
}
