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
             .takes_value(true))
        .get_matches();

    let seed = match matches.parsed_value_of("seed") {
        Some(n) => n,
        None => {
            println!("Seed not provided.");
            return;
        },
    };

    let count = match matches.parsed_value_of("count") {
        Some(n) => n,
        None => {
            println!("Count not provided.");
            return;
        }
    };

    // My evil plan was to use matches.occurrences_of("operation") to get the operations
    // required to actually execute this, but the ugly unfortunateness of the afternoon
    // is that the method in question returns a number, not the actual occurrences. This 
    // is unfortunate. As a stop-gap, because I'm not planning to fork and submit a pull 
    // request for this (at least not this afternoon!), I'm just going to take everything 
    // that can be parsed as an instruction and call it one.
    //
    // My plan did not work. The following code works, theoretically, but the arguments 
    // required to make use of this code will throw an error in clap:
    // 
    //      let instructions: Vec<Instruction> = std::env::args()
    //          .filter_map(|arg| arg.parse().ok())
    //          .collect();
    //
    // So... I'm going to have to have an argument named "operations" instead that takes 
    // all of the operations as one chunk. I can then split the string they return to 
    // get the individual instructions.

    let operations: Vec<Operation> = match matches.value_of("operations") {
        Some(input) => input.split(" ").filter_map(|s| s.parse().ok()).collect(),
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
