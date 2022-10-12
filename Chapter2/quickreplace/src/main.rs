use std::env;
use text_colorizer::*;
use std::fs;
use regex::Regex;

#[derive(Debug)]
struct Arguments {
    target: String,
    replacement: String,
    filename: String,
    output: String,
}

fn print_usage() {
    eprintln!("{} - change occurances of one string into another", "quickreplace".green());
    eprintln!("Usage: quickreplace <target> <replacement> <input> <output>");
}

fn replace(target: &str, replacement: &str, text: &str) -> Result<String, regex::Error> {
    let regex = Regex::new(target)?;
    return Ok(regex.replace_all(text, replacement).to_string());
}

fn parse_args() -> Arguments {
    // let args: Vec<String> = env::args().skip(1).collect();
    let args: Vec<_> = env::args().skip(1).collect();

    if args.len() != 4 {
        print_usage();
        std::process::exit(1);
    }

    Arguments {
        target: args[0].clone(),
        replacement: args[1].clone(),
        filename: args[2].clone(),
        output: args[3].clone(),
    }
}

fn main() {
    let args = parse_args();
    println!("{:?}", args);

    let data = match fs::read_to_string(&args.filename) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("{} failed to read file {}: {:?}", "Error".red().bold(), args.filename, e);
            std::process::exit(1);
        }
    };

    let replaced_data = match replace(&args.target, &args.replacement, &data) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("{} failed to replace text: {:?}", "Error".red().bold(), e);
            std::process::exit(1);
        }
    };

    match fs::write(&args.output, &replaced_data) {
        Ok(_) => {},
        Err(e) => {
            eprintln!("{} failed to write file {}: {:?}", "Error".red().bold(), args.output, e);
            std::process::exit(1);
        }
    };

}