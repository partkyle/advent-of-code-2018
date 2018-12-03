extern crate getopts;

use std::io::prelude::*;

use std::collections::HashSet;
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

    let mut frequencies = HashSet::new();
    let mut freq = 0;
    frequencies.insert(freq);
    loop {
        let f = File::open(file).expect("File not found");
        let r = BufReader::new(f);

        for line in r.lines() {
            let num = handle_frequency_line(line)?;
            freq += num;

            if frequencies.contains(&freq) {
                return Ok(freq);
            }

            frequencies.insert(freq);
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optflag("h", "help", "print this help menu");
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

    match handle_file(&file) {
        Ok(total) => { println!("total: {}", total) }
        Err(err) => {
            println!("error: {}", err);
            process::exit(1);
        }
    }
}
