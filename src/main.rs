extern crate clap;

use clap::{App, Arg};
use std::str::FromStr;

/// Represents the arithmetic operation to be applied, but does not also
/// store the value to be used in the operation, which I consider to be 
/// a shortcoming. However, I don't have the time or the inclination 
/// right now to add the value to these enum variants, so that's going to 
/// just have to be an enhancement. The problem would mainly be in parsing.
enum Operation {
    Add(f64),
    Subtract(f64),
    Multiply(f64),
    Divide(f64),
}

impl Operation {
    // Apply is a method of Operation and should have 
    // gone here instead of in the impl block for 
    // the FromStr trait. >.>
    fn apply(&self, n: f64) -> f64 {
        match self {
            &Operation::Add(value) => n + value,
            &Operation::Subtract(value) => n - value,
            &Operation::Multiply(value) => n * value,
            &Operation::Divide(value) => n / value,
        }
    }
}

impl FromStr for Operation {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Operation, &'static str> {
        // If we are unable to figure out the value for the 
        // operation, return the error early.
        let value = match s[1..].parse() {
            Ok(value) => value,
            Err(_) => return Err("Unable to parse operation value."),
        };

        // Here we have to parse just the first character in 
        // the command string, so I'm gonna use a slice to 
        // get that (as an &str). Kind of equivalent to a 
        // substring in everything else I've ever done.
        match &s[..1] {
            "+" => Ok(Operation::Add(value)),
            "-" => Ok(Operation::Subtract(value)),
            "*" => Ok(Operation::Multiply(value)),
            "/" => Ok(Operation::Divide(value)),
            _ => Err("Not a valid operation (should start with + - * /)."),
        }
    }
}

pub fn main() {
    let matches = App::new("Recurrence Relationships")
        .version("0.0.3")
        .author("J/A <archer884@gmail.com>")
        .about("Generates numeric series based on recurrence relationships.")
        .arg(Arg::new("seed")
             .short("s")
             .long("seed")
             .help("Sets the first term of the series.")
             .takes_value(true))
        .arg(Arg::new("count")
             .short("c")
             .long("count")
             .help("Sets the length of the series.")
             .takes_value(true))
        .arg(Arg::new("operations")
             .short("o")
             .long("operations")
             .help("Defines the operations used to create the series, e.g. \"+3 *2\"")
             .takes_value(true)
             .multiple(true))
        .get_matches();

    let seed = match matches.value_of("seed").and_then(|seed| seed.parse().ok()) {
        Some(n) => n,
        None => {
            println!("Seed not provided.");
            return;
        },
    };

    let count = match matches.value_of("count").and_then(|count| count.parse().ok()) {
        Some(n) => n,
        None => {
            println!("Count not provided.");
            return;
        }
    };

    // As you can see, kbknapp has updated clap-rs with the ability to read multiple 
    // values for a single key! This is wonderful news--at least for me, since I have 
    // this goofy tendency to write code that requires that feature for some reason.
    //
    // ...Don't ask, ok? 
    //
    // All right, the last time around had something to do with a database seeder that 
    // needed the ability to add arbitrary assemblies to its list at runtime.
    //
    // Anyway, it works now: note the call below has changed to `values_of()`
    let operations: Vec<Operation> = match matches.values_of("operations") {
        Some(values) => values.iter().filter_map(|s| s.parse().ok()).collect(),
        None => {
            println!("Unable to parse operations.");
            return;
        },
    };

    let mut i = seed;
    for _ in 0..count {
        println!("{}", i);
        i = operations.iter().fold(i, |a,b| b.apply(a));
    }
}

#[cfg(test)]
mod test {
    use super::{Instruction, Operation};

    #[test]
    fn can_parse_instruction() {
        assert!("+3".parse::<Instruction>().is_ok());
    }
}
