// core of the application

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use std::fs::File;
use std::io::BufReader;
use std::option::Option;
use std::path::Path;

#[derive(Serialize, Deserialize)]
pub struct WordData {
    data: HashMap<String, HashMap<String, Vec<String>>>,
}

impl WordData {
    pub fn generate_from_json(json_path: &str) -> WordData {
        // generate a WordData struct from a json
        let path = Path::new(json_path);
        let file = File::open(path).unwrap();
        let reader = BufReader::new(file);
        serde_json::from_reader(reader).unwrap()
    }

    pub(crate) fn get_value(&self, word: &String, lang: &String) -> Option<&Vec<String>> {
        // if the language and the word exits in the dataset then return the lang otherwise Option::None
        self.data.get(lang)?.get(word)
        // self.data.get("English")?.get("word")
    }

    pub(crate) fn word_analysis(&self, word: &String, lang: &String) -> Option<&Vec<String>> {
        self.get_value(word, lang)
    }

    pub fn simple_analysis(
        &self,
        words: &Vec<String>,
        lang: &String,
    ) -> (HashMap<&String, usize>, usize) {
        self.complex_analysis(words, lang, &|_,_| true)
    }

    /// does a complex analysis of the input words, and filters counting the words that pass the predicate
    pub fn complex_analysis(&self, words: &Vec<String>, lang: &String, predicate: & dyn Fn(&String, &String) -> bool) -> (HashMap<&String, usize>, usize) {
        // if the language is not in  the data base we can early return
        if !self.data.contains_key(lang) {
            return (HashMap::new(), words.len());
        }

        let mut output = HashMap::new();
        let mut not_coded = 0;

        // iterate over the set of input words
        for word in words {
            // find the lowercase word
            let lowered_word = word.to_lowercase();

            if let Some(p) = self.get_value(&lowered_word, lang) {
                for x in p.iter() {
                    if predicate(&lowered_word, x) {
                        output.entry(x).and_modify(|count| *count += 1).or_insert(1);
                    }
                }
            }
        }

        (output, not_coded)
    }
}
