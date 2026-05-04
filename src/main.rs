// Create a function which returns the number of true values there are in an array.
// https://edabit.com/challenges

#[allow(dead_code)]
fn countTruths(input: Vec<bool>) -> u16 {
    let truths = input.iter().filter(|i| **i==true).count() as u16;
    println!("found truths: {truths}");
    return truths;
}

// Write a function that takes in a string and returns a function that returns it.
#[allow(dead_code)]
fn redundant (input: String) -> impl Fn() -> String {
    move || input.to_string() // move forces ownership of input. idk why...
    // || is a closure in rust. Anything after it is the function body.
}

// Create a function that takes a variable number of arguments, each argument representing the number of items in a group.
// The function should return the number of permutations (combinations) of choices you would have if you selected one item from each group.
#[allow(dead_code)]
fn combinations(input: Vec<i16>) -> i16 {
    input.iter().fold(1, |acc, x| acc * x) // just like reduce in ts/js
}

// how many cars can you build
#[allow(dead_code)]
fn cars(wheels: i16, car_bodies: i16, figures: i16) -> i16 {
    let from_wheels = wheels / 4;
    let from_bodies = car_bodies;
    let from_figures = figures / 2;

    *[from_wheels, from_bodies, from_figures]
        .iter()
        .min()
        .expect("someting went wrong")
}

// return an array with all odd and even nums added up.
#[allow(dead_code)]
fn totalNums(input: Vec<i16>) -> Vec<i16>{
    let odd_total = input.iter().filter(|&x| {x % 2 == 0}).sum();
    let even_total = input.iter().filter(|&x| {x % 2 == 1}).sum();
    [odd_total, even_total].to_vec()
}

// uncensor("Wh*r* d*d my v*w*ls g*?", "eeioeo") ➞ "Where did my vowels go?"
fn uncensor(phrase: String, missing_vowels: String) -> String {
    let mut i = 0;
    let mut output = phrase.clone();
    while i < missing_vowels.len() as i16{
        output.replace("*", missing_vowels[i]); // cant do this in rust because string as stored in utf8 buffer. i will have to iterate over the entire thing using rust methods instead of loops
        i+=1;
    }
    output.to_string()
}

fn main() {
}