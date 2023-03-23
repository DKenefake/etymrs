use std::collections::{HashMap, HashSet};

pub(crate) struct GeneralAnalysis {
    lang_counter: HashMap<String, f64>,
}

impl GeneralAnalysis {
    pub(crate) fn from_count(lang_counter: HashMap<&String, usize>) -> GeneralAnalysis {
        let sum = lang_counter.values().sum::<usize>() as f64;

        let lang_counter = lang_counter
            .iter()
            .map(|(key, value)| (key.to_string(), *value as f64 / sum))
            .collect();

        GeneralAnalysis { lang_counter }
    }

    pub(crate) fn get_sorted_lang(&self) -> Vec<(String, f64)> {
        // collect the lang_counter into a sorted vector
        let mut lang_counter: Vec<(String, f64)> = self.lang_counter.iter().map(|(key, value)| (key.to_string(), *value)).collect();
        lang_counter.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        lang_counter
    }
}
