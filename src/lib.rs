#![crate_name = "etymrs"]

pub mod core;
pub mod stats;
pub mod word;

pub use crate::core::*;
pub use crate::stats::*;
pub use crate::word::*;

#[cfg(test)]
mod tests {

    use crate::core::WordData;
    use crate::stats::GeneralAnalysis;
    use crate::*;
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

        println!("{}", a.1)
    }

    #[test]
    fn test_general_analysis(){
        let word_data = WordData::generate_from_json("data//words.json");
        let s = |x: &str| String::from(x);

        // read in a sample text
        let data = fs::read_to_string("data//shakespeare.txt").expect("Unable to read file");

        // tokenize and run
        let input = data.split(" ").map(String::from).map(|x| x.replace(&['(', ')', ',', '\"', '.', ';', ':', '\''],"")).collect();

        let (a,b) = word_data.simple_analysis(&input, &s("English"));

        let gen_anal = GeneralAnalysis::from_count(a);

        for (lang, value) in gen_anal.get_sorted_lang(){
            println!("{} : {}", lang, value);
        }
    }

    #[test]
    fn test_performance() {
        // create the word_data struct from the json file
        let word_data = WordData::generate_from_json("data//words.json");

        // read in a sample text
        let data = fs::read_to_string("data//book.txt").expect("Unable to read file");

        // tokenize and run
        let input = data.split(" ").map(String::from).map(|x| x.replace(&['(', ')', ',', '\"', '.', ';', ':', '\''],"")).collect();

        // get an initial time
        let start = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();

        // run the analysis
        let (results, not_counted) = word_data.complex_analysis(&input, &String::from("English"), &|x,y| ! (x.eq("or") && y.eq("Latin")));

        // get an end time
        let end = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();

        println!(
            "Time to solve is {} ms with {} ns per word, {} not counted words",
            (end - start) / 1_000_000u128,
            (end - start) / (input.len() as u128),
            not_counted
        );
    }
}
