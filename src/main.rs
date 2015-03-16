extern crate clap;

use clap::{App, Arg};
use std::str::FromStr;

/// Represents the arithmetic operation to be applied, but does not also
/// store the value to be used in the operation, which I consider to be 
/// a shortcoming. However, I don't have the time or the inclination 
/// right now to add the value to these enum variants, so that's going to 
/// just have to be an enhancement. The problem would mainly be in parsing.
enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
}

impl FromStr for Operation {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Operation, &'static str> {
        match s {
            "+" => Ok(Operation::Add),
            "-" => Ok(Operation::Subtract),
            "*" => Ok(Operation::Multiply),
            "/" => Ok(Operation::Divide),
            _ => Err("No dice."),
        }
    }
}

struct Instruction {
    op: Operation,
    value: f64,
}

impl Instruction {
    fn apply(&self, n: f64) -> f64 {
        match self.op {
            Operation::Add => n + self.value,
            Operation::Subtract => n - self.value,
            Operation::Multiply => n * self.value,
            Operation::Divide => n / self.value,
        }
    }
}

impl FromStr for Instruction {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Instruction, &'static str> {
        let op = match s[0..1].parse() {
            Ok(op) => op,
            _ => return Err("Unable to parse operation."),
        };

        let value = match s[1..].parse() {
            Ok(value) => value,
            _ => return Err("Unable to parse value."),
        };

        Ok(Instruction {
            op: op,
            value: value,
        })
    }
}

pub fn main() {
    let matches = App::new("Recurrence Relationships")
        .version("0.0.1")
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
        .arg(Arg::new("operation")
             .short("o")
             .long("operation")
             .help("Defines an operation used to create the series, e.g. \"+3\"")
             .multiple(true)
             .takes_value(true))
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

    // My evil plan was to use matches.occurrences_of("operation") to get the operations
    // required to actually execute this, but the ugly unfortunateness of the afternoon
    // is that the method in question returns a number, not the actual occurrences. This 
    // is unfortunate. As a stop-gap, because I'm not planning to fork and submit a pull 
    // request for this (at least not this afternoon!), I'm just going to take everything 
    // that can be parsed as an instruction and call it one.

    let instructions: Vec<Instruction> = std::env::args().filter_map(|arg| arg.parse().ok()).collect();

    if instructions.len() == 0 {
        println!("Instruction(s) not provided.");
        return;
    }

    let mut i = seed;
    for _ in 0..count {
        println!("{}", i);
        i = instructions.iter().fold(i, |a,b| b.apply(a));
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
