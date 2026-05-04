// Create a function which returns the number of true values there are in an array.
// https://edabit.com/challenges

#[warn(unused)]
fn countTruths(input: Vec<bool>) -> u16 {
    let truths = input.iter().filter(|i| **i==true).count() as u16;
    println!("found truths: {truths}");
    return truths;
}

// Write a function that takes in a string and returns a function that returns it.
fn redundant (input: String) -> impl Fn() -> String {
    move || input.to_string() // move forces ownership of input. idk why...
    // || is a closure in rust. Anything after it is the function body.
}

fn main() {
    let returnedStr = redundant("hello".to_string());
    println!("{}", returnedStr()); 
}