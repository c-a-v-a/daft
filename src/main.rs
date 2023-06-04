use std::fs;
use std::fs::File;
use std::io::{Write, BufRead, BufReader};

mod lexer;
mod parser;

use lexer::Token;
use parser::Heading;

use clap::Parser;

#[derive(Parser)]
struct Args {
    /// Minimum heading level to put in table of contents
    #[arg(long, default_value_t = 1)]
    min_level: usize,

    /// Maximum heading level to put in table of contents
    #[arg(long, default_value_t = 6)]
    max_level: usize,

    /// Markdown file to read from
    input: String,

    /// Write table of contents into file
    #[arg(short, long, default_value_t = false)]
    write: bool,
}

fn stringify(parsed: Vec<Heading>, min_level: usize) -> Vec<String> {
    parsed
        .iter()
        .map(|heading| {
            let indent_size = heading.level - min_level;
            let indent = vec![' '; indent_size * 4].iter().collect::<String>();
            format!("{}* [{}]({})", indent, heading.text, heading.link)
        })
        .collect::<Vec<String>>()
}

fn write_toc(input_path: String, toc_string: String) -> std::io::Result<()> {
    let input_file = File::open(&input_path)?;
    let mut temp_file = File::create("daft-temp")?;

    let buf_read = BufReader::new(input_file);
    let lines = buf_read.lines();

    for line in lines {
        println!("{}", 112);
        let line: String = line?;


        if line == "<!-- TOC -->" {
            write!(temp_file, "# Table of contents\n")?;
            write!(temp_file, "{}", toc_string)?;
        } else {
            write!(temp_file, "{}\n", line)?;
        }
    }

    fs::copy("daft-temp", input_path)?;
    fs::remove_file("daft-temp")?;

    Ok(())
}

fn main() {
    let args = Args::parse();
    let file = File::open(&args.input);

    match file {
        Ok(file) => {
            let buf = BufReader::new(file);
            let lines = buf.lines();

            let tokenized: Vec<Vec<Token>> = lines
                .map(|x| x.unwrap_or_default())
                .filter(|x| !x.is_empty())
                .map(|x| lexer::tokenize_line(x))
                .collect();

            let parsed = parser::parse(tokenized);
            let filtered = parsed
                .into_iter()
                .filter(|x| x.level >= args.min_level && x.level <= args.max_level)
                .collect::<Vec<Heading>>();

            let toc_string = stringify(filtered, args.min_level).join("\n");

            if args.write {
                match write_toc(args.input, toc_string) {
                    Ok(_) => println!("Succesfully wrote to file"),
                    Err(_) => println!("Could not write to file")
                };
            } else {
                println!("{}", toc_string);
            }

        }
        Err(_) => println!("Could not open the file."),
    };
}
