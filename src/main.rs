use clap::Parser;
use etymrs::core::WordData;
use std::fs;
use std::fs::File;
use std::io::{BufWriter, Write};

// Simple program to great a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short = 'i', long)]
    readfile: String,
    #[arg(short = 'o', long, default_value = "output.txt")]
    outfile: String,
    #[arg(short = 'l', long, default_value = "English")]
    language: String,
    #[arg(short = 'd', long, default_value = "data//words.json")]
    data_path: String,
}

fn main() {
    // load in the
    let args = Args::parse();

    // load in the json database
    let word_data = WordData::generate_from_json(&args.data_path);

    // load in the text database
    let data = fs::read_to_string(&args.readfile).expect("Unable to read file");

    // tokenize the input
    let input = data.split(" ").map(|x| String::from(x)).collect();

    // run the analysis
    let (results, not_counted) = word_data.simple_analysis(&input, &args.language);

    // write the file
    let f = File::create(&args.outfile).expect("Unable to write file");
    let mut f = BufWriter::new(f);

    for (k, v) in results {
        let rendered_line = String::from(format!("{} : {} \n", k, v).as_str());
        f.write(rendered_line.as_bytes())
            .expect("Unable to write data");
    }
}
