use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn parse_markdown_file(filename: &str) {
    print_short_banner();
    println!("[ INFO ] Trying to parse {}...", filename);

    let path = Path::new(filename);
    let fp = File::open(&path).expect("[ ERROR ] Failed to open file!");
    let mut _ptag: bool = false;
    let mut _htag: bool = false;
    let mut tokens: Vec<String> = Vec::new();
    let reader = BufReader::new(fp);

    for line in reader.lines() {
        let contents = line.unwrap();
        let mut first_char: Vec<char> = contents.chars().take(1).collect();
        let mut output_line = String::new();

        match first_char.pop() {
            Some('#') => {}
            _ => {}
        }
    }
}

fn print_short_banner() {
    println!("{}", get_title());
}

fn print_long_banner() {
    print_short_banner();
    let mut written_by = String::from("Written by: ");
    written_by.push_str(env!("CARGO_PKG_AUTHORS"));
    let mut homepage = String::from("Homepage: ");
    homepage.push_str(env!("CARGO_PKG_HOMEPAGE"));
    let usage = String::from("Usage: tinymd <somefile>.md");
    println!("{}", written_by);
    println!("{}", homepage);
    println!("{}", usage);
}

fn get_title() -> String {
    let mut title = String::from(env!("CARGO_PKG_NAME"));
    title.push_str(" (v");
    title.push_str(env!("CARGO_PKG_VERSION"));
    title.push_str("), ");
    title.push_str(env!("CARGO_PKG_DESCRIPTION"));
    title
}

fn usage() {
    print_long_banner();
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    match args.len() {
        2 => parse_markdown_file(&args[1]),
        _ => {
            println!("[ ERROR ] Invalid invocation");
            usage();
        }
    }
}
