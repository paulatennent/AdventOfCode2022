mod days;

use clap::Parser;
use days::*;

#[derive(Parser, Default, Debug)]
struct Arguments {
    day: i32,
}

fn main() {
    let args = Arguments::parse();

    match args.day {
        1 => day_1::day_1(),
        _ => println!("Opening your advent calendar early are you?!? Blasphemous!"),
    }
}
