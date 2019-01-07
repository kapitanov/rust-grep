use std::env;
use std::io::BufRead;

mod grep;
use crate::grep::*;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let config = Config::new(&args).unwrap_or_else(err_handler);
    if config.show_help() {
        print_help();
        return;
    }

    let runner = config.create_runner();

    for line in std::io::stdin().lock().lines() {
        let s = line.unwrap();
        if runner.is_match(&s) {
            println!("{}", s);
        }
    }
}

fn err_handler<T>(err: String) -> T {
    eprintln!("ERROR!");
    eprintln!("{}", err);
    std::process::exit(-1)
}

fn print_help() {
    println!("#################################################################");
    println!("#                                                               #");
    println!("# RUSTGREP - a partial implementation of 'grep' written in Rust #");
    println!("#                                                               #");
    println!("#################################################################");
    println!();
    println!("Usage:");
    println!("  rustgrep [OPTIONS] PATTERN [PATTERN...]");
    println!();
    println!("Options:");
    println!("  -i, --ignore-case  - turn \"ignore case\" mode on");
    println!("  -r, --regex        - turn \"regular expression\" mode on");
    println!("  -v, --verbose      - enable verbose output");
    println!("  -?, -h, --help     - show help and exit");
    println!();
}
