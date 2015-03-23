Challenge #206 [Easy]
=====================

> [Recurrence Relations, Part 1](http://www.reddit.com/r/dailyprogrammer/comments/2z68di/20150316_challenge_206_easy_recurrence_relations/)

## Operation

The `Operation` enum encapsulates all of the information required to apply an arithmetic operation to a value. It has variants representing add, subtract, multiply, and divide operations, and each variant stores a value to be used as its second operand. 

## Instruction

This version has removed the `Instruction` struct entirely, rolling that functionality into the `Operation` enum.

## CLAP

Clap is, apparently, not *just* a disease; it's also a command line argument parsing library. I'm sure that fleeting similarity between the name of the library and the initials of those randomly-chosen words is pure coincidence.

The author of clap, as it turns out, has actually implemented the one feature I thought it was missing since the original version of this application. Probably dumb luck on my part, but I'm going to pretend it's because he just felt so sorry for my silly /r/dailyprogrammer solution. :)

Initially, there was a little bug in clap that prevented me using the new hotness for this program, but the author quite graciously merged a pull request from me (my first ever on github!) and so everything is working perfectly at this point.
