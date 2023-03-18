use std::collections::{HashMap, HashSet};

struct GeneralAnalysis{
    lang_counter: HashMap<String, usize>
}

impl GeneralAnalysis {

    fn from(lang_counter: HashMap<String, usize>) -> GeneralAnalysis{
        GeneralAnalysis{ lang_counter}
    }
}