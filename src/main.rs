// بسم الله الرحمن الرحيم

mod engine;

use std::{env, fs};

const APP_NAME: &str = "hwl";
const VERSION: &str = "v0.1.0 Experimental";

fn display_help() {
    println!(
        r#"{} - Command-line Engine that transliterates Arabic text to English (Experimental)
    
Usage: {} [TEXT] [OPTIONS]

Options:
    -V, --version         Print the application's version
    -F, --file <filename> Transliterate a file's contents
    -H, --help            Show this help message

Usage Examples:

    $ {} كَيفَ حالُكَ يا أَخِي؟
    $ {} --file content.txt
    $ {} -V
    
Amad Project: https://codeberg.org/amad"#,
        APP_NAME, APP_NAME, APP_NAME, APP_NAME, APP_NAME
    );
}

fn main() {
    let mut args: Vec<String> = env::args().collect();

    match args.len() {
        1 => display_help(),
        _ => {
            if args[1] == "--file" || args[1] == "-F" {
                let filename = &args[2];
                let contents = fs::read_to_string(filename).expect("FAILED TO READ FILE!");

                let result = engine::transliterate(&contents);
                println!("{}", result);
            } else if args[1] == "--help" || args[1] == "-H" {
                display_help();
            } else if args[1] == "--version" || args[1] == "-V" {
                println!(
                    "{} {}\nAmad Project: https://codeberg.org/amad",
                    APP_NAME, VERSION
                );
            } else {
                args.remove(0);
                let words = args.join(" ");

                let result = engine::transliterate(&words);
                println!("{}", result);
            }
        }
    }
}

// تم بحمد الله