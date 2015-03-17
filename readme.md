Challenge #206 [Easy] Recurrence Relations, part 1
==================================================

From [/r/dailyprogrammer](http://www.reddit.com/r/dailyprogrammer/comments/2z68di/20150316_challenge_206_easy_recurrence_relations/)

## Operation

The `Operation` enum represents nothing other than +, -, *, and /. When you combine it with the numeric value stored in the `Instruction` struct, it becomes useful, but not until then. :\

## Instruction

The `Instruction` struct combines an `Operation` enum and an `f64` value to provide all the information required by the program to create the next value in the sequence. I would prefer to have combined this struct and the `Operation` enum into a single enum along the lines of the following...

    enum Operation {
        Add(f64),
        Subtract(f64),
        ...
    }

...but I only get so much lunch break, so I didn't want to take a lot of time messing around with how to parse that. Mostly I just felt like the parsing operation would be kinda ugly.

## CLAP

Clap is, apparently, not *just* a disease; it's also a command line argument parsing library. I'm sure that fleeting similarity between the name of the library and the initials of those randomly-chosen words is pure coincidence.

Like most things in Rust land, I gotta figure it's still under development, so take my comments with a grain of salt (or three). I was planning to use `matches.occurrences_of("operation")` to get the operations required to actually execute this, but the ugly unfortunateness of the afternoon was that the method in question returns a number, not the actual occurrences. As a stop-gap, I tried to take everything that can be parsed as an `Instruction` and call it one using the code below...

    let instructions: Vec<Instruction> = std::env::args()
        .filter_map(|arg| arg.parse().ok())
        .collect();

...but that didn't work out because the arguments required to make that work actually throw an error in clap because clap won't accept input that doesn't map to one of the items defined for the app.

The solution I eventually went with was just to pass all the required operations as a single string on a single argument.
