use std::fs::File;
use std::io::Write;
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
            Some('#') => {
                if _ptag {
                    _ptag = false;
                    output_line.push_str("</p>\n");
                }

                if _htag {
                    _htag = false;
                    output_line.push_str("</h1>\n");
                }

                _htag = true;
                output_line.push_str("<h1>");
                output_line.push_str(&contents[2..]);
            }
            _ => {
                if _htag {
                    _htag = false;
                    output_line.push_str("</h1>\n");
                }

                if !_ptag {
                    _ptag = true;
                    output_line.push_str("<p>");
                }

                output_line.push_str(&contents);
            }
        };

        if _htag {
            _htag = false;
            output_line.push_str("</h1>\n");
        }

        if _ptag {
            _ptag = false;
            output_line.push_str("</p>\n");
        }

        if output_line != "<p></p>\n" {
            tokens.push(output_line);
        }
    }

    let mut output_filename = String::from(&filename[..filename.len() - 3]);
    output_filename.push_str(".html");

    let mut output_file =
        File::create(output_filename).expect("[ ERROR ] Could not create output file!");
    for line in &tokens {
        output_file
            .write_all(line.as_bytes())
            .expect("[ ERROR ] Could not write to output file!");
    }

    println!("[ INFO ] Parsing complete!");
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
