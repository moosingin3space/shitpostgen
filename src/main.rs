extern crate getopts;
use getopts::Options;
use std::env;

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options] TEXT TO SHITPOST", program);
    print!("{}", opts.usage(&brief));
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    let mut opts = Options::new();
    opts.optflag("h", "help", "print this help menu");
    opts.optflag("3", "3d", "print a 3-D ASCII shitpost graphic!");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };

    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }

    let mut shitpost_string = matches.free.iter().fold(String::new(), |r, c| r + c.as_ref() + " ");
    shitpost_string.pop();
    let shitpost_string = shitpost_string.to_uppercase();
    let first_char = shitpost_string.chars().next().expect("shitpost must have content!");

    if matches.opt_present("3") {
        let len = (shitpost_string.len() * 2) - 2;
        for (i, ch) in shitpost_string.chars().enumerate() {
            if i == 0 || i == shitpost_string.len()-1 {
                // Print the full string with spaces
                for ch in shitpost_string.chars() {
                    print!("{} ", ch);
                }

                if i == shitpost_string.len()-1 {
                    print!("{: >1$}", ch, i-1);
                }
                println!("");
            } else {
                // print just the char with padding
                println!("{0}{0: >1$}{0: >2$}", ch, len, i);
            }
        }
        for (i, ch) in shitpost_string.chars().enumerate().skip(1) {
            if i == shitpost_string.len()-1 {
                print!("{: >1$} ", first_char, shitpost_string.len());
                for ch in shitpost_string.chars().skip(1) {
                    print!("{} ", ch);
                }
                println!("");
            } else {
                println!("{0: >1$}{0: >2$}{0: >3$}", ch, i+1, len, shitpost_string.len()-i-1);
            }
        }
    } else {
        for ch in shitpost_string.chars() {
            print!("{} ", ch);
        }
        println!("");

        for ch in shitpost_string.chars().skip(1) {
            println!("{}", ch)
        }
    }
}
