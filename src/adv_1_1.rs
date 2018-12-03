extern crate getopts;

use std::io::prelude::*;

use std::env;
use std::fs::File;
use std::io::BufReader;
use std::process;

use getopts::Options;

use advent::lib::handle_frequency_line;

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options]", program);
    print!("{}", opts.usage(&brief));
}

fn handle_file(file: &str) -> Result<i32, String> {
    let f = File::open(file).expect("File not found");
    let r = BufReader::new(f);

    let mut total = 0i32;
    for line in r.lines() {
        total += handle_frequency_line(line)?;
    }

    Ok(total)
}

fn handle_file_elegantly(file: &str) -> Result<i32, String> {
    let f = File::open(file).expect("File not found");
    let r = BufReader::new(f);

    r.lines().map(handle_frequency_line).sum()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optflag("h", "help", "print this help menu");
    opts.optflag("e", "elegant", "use the elegant solution");
    opts.optflagopt("f", "file", "input file to read for frequencies", "FILE");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };

    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }

    let file = match matches.opt_str("file") {
        Some(f) => f,
        None => {
            println!("FILE is required");
            process::exit(1);
        }
    };

    let handler = if matches.opt_present("e") {
        handle_file_elegantly
    } else {
        handle_file
    };

    match handler(&file) {
        Ok(total) => { println!("total: {}", total) }
        Err(err) => {
            println!("error: {}", err);
            process::exit(1);
        }
    }
}
