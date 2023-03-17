mod word;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

use std::fs::File;
use std::io::BufReader;
use std::option::Option;
use std::path::Path;

#[derive(Serialize, Deserialize)]
struct WordData {
    data: HashMap<String, HashMap<String, Vec<String>>>,
}

impl WordData {
    fn generate_from_json(json_path: &str) -> WordData {
        // generate a WordData struct from a json
        let path = Path::new(json_path);
        let file = File::open(path).unwrap();
        let reader = BufReader::new(file);
        serde_json::from_reader(reader).unwrap()
    }

    fn get_value(&self, word: &String, lang: &String) -> Option<&Vec<String>> {
        // if the language and the word exits in the dataset then return the lang otherwise Option::None
        self.data.get(lang)?.get(word)
        // self.data.get("English")?.get("word")
    }

    fn logic(&self, output: &mut HashMap<String, usize>, sub_lang: &String) -> () {
        output
            .entry(sub_lang.clone())
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }

    fn anal(&self, words: &Vec<String>, lang: &String) -> (HashMap<String, usize>, usize) {
        // initialize the output object
        let mut output = HashMap::new();
        let mut not_coded = 0;

        let unique_words = words.iter().collect::<HashSet<&String>>();

        let unique_pairs = unique_words
            .iter()
            .map(|&x| (x, self.get_value(x, lang)))
            .collect::<HashMap<&String, Option<&Vec<String>>>>();

        for word in words {
            if let Some(Some(p)) = unique_pairs.get(word) {
                p.iter().for_each(|x| self.logic(&mut output, x));
            } else {
                not_coded += 1;
            }
        }

        (output, not_coded)
    }

    fn anal_2(&self, words: &Vec<String>, lang: &String) -> (HashMap<&String, usize>, usize) {
        // initialize the output object
        let mut output = HashMap::new();
        let mut not_coded = 0;

        for word in words {
            if let Some(p) = self.get_value(word, lang){
                for x in p.iter(){
                    output
                        .entry(x)
                        .and_modify(|count| *count += 1)
                        .or_insert(1);
                }
            }else{
                not_coded += 1;
            }
        }

        (output, not_coded)
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    fn read_json() {
        let word_data = WordData::generate_from_json("src//words.json");

        for v in word_data
            .get_value(&String::from("drink"), &String::from("English"))
            .unwrap()
        {
            println!("{}", v)
        }
    }

    #[test]
    fn test_anal(){
        let word_data = WordData::generate_from_json("src//words.json");
        let s = |x:&str| String::from(x);

        let input = "I am a golden god I need my tools".split(" ").map(|x| s(x)).collect();

        let a = word_data.anal(&input, &s("English"));

        for (key, value) in a.0{
            println!("{} : {}", key, value);
        }
    }

    #[test]
    fn test_perf(){
        let word_data = WordData::generate_from_json("src//words.json");

        let data =  fs::read_to_string("src//book.txt").expect("Unable to read file");

        let s = |x:&str| String::from(x);

        let input = data.split(" ").map(|x| s(x)).collect();
        let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();

        let a = word_data.anal_2(&input, &s("English"));
        let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();

        println!("{}",end - start);

        for(key, value) in a.0{
            println!("{} : {}", key, value);
        }
        println!("{} {}",input.len(),a.1);
    }
}
