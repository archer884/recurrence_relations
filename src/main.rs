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
        .arg(Arg::new("instructions")
             .short("i")
             .long("instructions")
             .help("Defines the instructions used to create the series, e.g. \"+3 *2\"")
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
    //
    // I should probably call the argument instructions instead of operations.

    let instructions: Vec<Instruction> = match matches.value_of("instructions") {
        Some(input) => input.split(" ").filter_map(|s| s.parse().ok()).collect(),
        None => {
            println!("Unable to parse instructions.");
            return;
        },
    };

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
