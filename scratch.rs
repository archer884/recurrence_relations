// Sample code describing how a version of this program could be written
// using just functions for the transformations instead of what I actually 
// went with (which I don't know how to describe just now). The main thing
// is that these are returned as boxed trait objects rather than as ... 
// you know. Static things.

fn main() {
    let mut x = 0;
    let y = get_operations();

    for _ in 0..10 {
        println!("{}", x);
        x = y.iter().fold(x, |a,b| b(a));
    }
}

fn get_operations() -> Vec<Box<Fn(u64) -> u64>> {
    vec![Box::new(|n| n * 2), Box::new(|n| n + 1)]
}
