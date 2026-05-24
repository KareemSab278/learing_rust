// doing rust challenges to code better.

use read_input::prelude::*;
use std::collections::HashMap;

// Temperature Converter
// Convert Celsius ↔ Fahrenheit from user input.
#[allow(dead_code)]
enum TempType {
    Celsius,
    Fahrenheit,
}
#[allow(dead_code)]
fn convert_temp(input: f32, tmp_type: TempType) -> f32 {
    // could have used char but char is always 4 bytes. str is 1 byte but can be bigger. for simple letter its 1b.
    // let input = input as f64; // i had no idea you coul do this....
    match tmp_type {
        TempType::Celsius => {
            return input * 1.8 + 32.0;
        }
        TempType::Fahrenheit => {
            return (input - 32.0) / 1.8;
        }
    }
}
// fn main() {
//     let temp_to_f = convert_temp(0.0, TempType::Celsius);
//     let temp_to_c = convert_temp(212.4, TempType::Fahrenheit);
//     println!("{}", temp_to_f);
//     println!("{}", temp_to_c);
// }

// ===

// Number Guessing Game
// Computer picks a number, user guesses until correct.
// Learn: loops, match, randomness.
#[allow(dead_code)]
fn guess_loop(random_int: u32) {
    loop {
        print!("Guess a number between 1 and 10: ");
        let input = input::<u32>().get();
        if random_int == input {
            println!("You won!");
            return;
        } else {
            println!("try again");
        }
    }
}

// fn main() {
//     let mut range = rand::thread_rng();
//     // unsigned int because only + ints for guesses between 1 and 10
//     let random_int: u32 = range.gen_range(0..11);
//     let _ = guess_loop(random_int);
// }

// ===
// FizzBuzz (Rust-style)
// Print numbers 1–100 with Fizz/Buzz rules.
// Learn: conditionals, loops.
#[allow(dead_code)]
fn fizz_buzz(mut i: u32) {
    if i > 0 {
        loop {
            if i % 3 == 0 && i % 5 == 0 {
                println!("{} - FizzBuzz", i);
            } else if i % 5 == 0 {
                println!("{} - Buzz", i);
            } else if i % 3 == 0 {
                println!("{} - Fizz", i);
            } else {
                println!("{} not fizz or buzz or fizzbuzz", i);
            }
            if i == 0 {
                break;
            }
            i -= 1;
        }
    }
}

// fn main() {
//     fizz_buzz(10);
// }

// 🟡 Level 2: Ownership & Collections

// Word Counter
// Read a sentence and count how many times each word appears.
// Learn: HashMap, borrowing, lifetimes (light).
#[allow(dead_code)]
fn find_words(sentence: &str) {
    let mut words_hmap = HashMap::new();
    for word in sentence.split_whitespace() {
        let count = words_hmap.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", words_hmap);
}
// fn main() {
//     find_words(
//         "The HashMap should be declared inside the function or passed as a mutable reference."
//     )
// }

// Reverse a String (Safely)
// Reverse a UTF-8 string correctly (no .chars().rev().collect() shortcut at first 😉).
// Learn: strings, iterators, Unicode.
#[allow(dead_code)]
fn reverse_str(input: &str) {
    let chars: Vec<char> = input.chars().collect(); // i can make a arr of chars from a str using chars() and collect() (collect turns iterators into collections like vec or hmap)
    let mut output: Vec<char> = [].to_vec(); // you have to set an empty vec using [].to_vec()
    for c in chars.iter().rev() {
        // iterate over vec using iter() and to reverse the iterator vec use rev() simpler than js...
        output.push(*c);
    }
    println!("output: {}", output.into_iter().collect::<String>()); // convert a vec into a str you gotta make it iter again since we used collect. then collect again ad turn into strign...
}

// fn main() {
//     let input = "12345";
//     reverse_str(input);
// }

// Todo List (CLI)
// Add, remove, and list tasks.
// Learn: structs, Vec, mutability.
#[allow(dead_code, unused_variables)]
#[derive(Debug)]
struct ToDo {
    id: u16,
    note: String,
}

impl ToDo {
    #[allow(dead_code, unused_variables)]
    fn add(todos: &mut Vec<ToDo>, id: u16, note: String) {
        todos.push(ToDo { note, id });
    }

    #[allow(dead_code)]
    fn ls(todos: Vec<ToDo>) {
        println!("your todo: {:?}", todos);
    }
}

// fn main() {
//     let mut todos: Vec<ToDo> = vec![]; // i forgot what the f that is...
//     // okay so arr is on stack vec is on heap which is why you can resize it. id prefer to avoid using vec as much as possible to stick to smoother perfomance but thats expert shit and im a silly jr dev lol
//     // The vec![] macro in Rust is used to create a new vector, which is a resizable array.
//     // You can initialize it with elements by placing them inside the brackets, like vec![1, 2, 3], or create an empty vector wit
//     ToDo::add(&mut todos, 1, "dont die".to_string());
//     ToDo::ls(todos);
// }

// 🟠 Level 3: Structs & Enums

// Simple Bank Account
// Deposit, withdraw, prevent overdrafts.
// Learn: methods, impl, error handling.

#[derive(Debug)]
#[allow(dead_code, unused_variables)]
struct Account {
    acc_id: u32,
    name: String,
    balance: f64,
}

// let max_value = arr.iter().max().unwrap();

impl Account {
    #[allow(dead_code, unused_variables)]
    fn create_acc(accounts: &mut Vec<Account>, name: String) {
        let mut ids = vec![];
        for account in accounts.iter() {
            // you would use iter here to loop over everything without taking any ownership
            ids.push(account.acc_id);
        }
        if ids.len() == 0 {
            ids.push(0);
        }
        let max_id = ids.iter().max().expect("failed to find max id");
        accounts.push(Account { acc_id: *max_id + 1, name, balance: 0.0 });
        println!("account created: {:?}", accounts);
    }

    #[allow(dead_code, unused_variables)]
    fn deposit(accounts: &mut Vec<Account>, acc_id: u32, amount: f64) {
        // find account by id and add amount to balance
        for account in accounts.iter_mut() {
            // iter_mut allows us to change the accounts in the vec
            if account.acc_id == acc_id {
                account.balance += amount;
                println!(
                    "deposited ${} to account {}. new balance: ${}",
                    amount,
                    acc_id,
                    account.balance
                );
                return;
            } else {
                println!("account {} not found", acc_id);
            }
        }
    }

    #[allow(dead_code, unused_variables)]
    fn withdraw(accounts: &mut Vec<Account>, acc_id: u32, amount: f64) {
        for account in accounts.iter_mut() {
            if account.acc_id == acc_id {
                if account.balance >= amount {
                    account.balance -= amount;
                    println!(
                        "withdrew ${} from account {}. new balance: ${}",
                        amount,
                        acc_id,
                        account.balance
                    );
                } else {
                    println!("insufficient funds for account {}", acc_id);
                }
                return;
            }
        }
    }

    #[allow(dead_code)]
    fn check_balance(accounts: &Vec<Account>, acc_id: u32) {
        for account in accounts.iter() {
            if account.acc_id == acc_id {
                println!("account {} balance: ${}", acc_id, account.balance);
                return;
            }
        }
        println!("account {} not found", acc_id);
    }
}

// fn main() {
//     let mut accounts: Vec<Account> = vec![]; // new vec on heap
//     Account::create_acc(&mut accounts, "bob".to_string());
//     Account::deposit(&mut accounts, 1, 100.0);
//     Account::withdraw(&mut accounts, 1, 30.0);
//     Account::withdraw(&mut accounts, 1, 80.0);
//     Account::check_balance(&accounts, 1);
// }

// Shape Area Calculator
// Enum for Circle, Rectangle, Triangle.
// Learn: enums, pattern matching.
#[derive(Debug)]
#[allow(dead_code, unused_variables)]
pub enum Shape {
    Circle(u32),
    Rectangle(u32, u32),
    Triangle(u32, u32, u32),
}

impl Shape {
    #[allow(dead_code, unused_variables)]
    pub fn calc_area(shape: Shape) {
        match shape {
            Shape::Circle(radius) => {
                let area = std::f64::consts::PI * (radius as f64).powi(2);
                println!("Circle area: {}", area);
            }
            Shape::Rectangle(length, width) => {
                let area = length * width;
                println!("Rectangle area: {}", area);
            }
            Shape::Triangle(base, height, _side) => {
                let area = 0.5 * (base as f64) * (height as f64);
                println!("Triangle area: {}", area);
            }
        }
    }
}


// Beginner Exercises

// Hello, Ownership
// Write a function that takes ownership of a String, modifies it, and returns it.
// Try calling the function twice with the same variable and observe the compiler error.
// Goal: Understand ownership and move semantics.

// fn edit_str(input: &str)-> String{
//     return input.to_string().to_uppercase();
// }
// fn main(){
//     let mut greeting: String = "Hello world!".to_string();
//     let output = edit_str(&greeting);
//     let output1 = edit_str(&greeting);
//     println!("{output}, {output1}");
// }

// Borrow Checker Fun
// Create a function that takes a reference to a vector of integers and returns the sum.
// Try modifying the vector inside the function while also returning a reference.
// Goal: Understand borrowing and mutable vs immutable references.

// fn sum_vec(input: Vec<i32>)-> i32{
//     let mut tot: i32 = 0;
//     for num in input{
//         tot += num
//     }
//     tot
// }

// fn main(){
//     let nums: Vec<i32> = [1,1,1,1,1,1].to_vec();
//     let output: i32 = nums.iter().sum();
//     println!("{output}");
// }

// Structs & Methods
// Define a struct called Rectangle with width and height.
// Implement a method area that returns the area.
// Implement another method can_hold(&self, other: &Rectangle) that returns true if self can fully contain other.

// struct Rectangle{
//     width: i32,
//     height: i32,
// }
// impl Rectangle{
//     fn calc_area(width: i32, height:i32)->i32{
//         return width * height;
//     }

//     fn can_hold(curr_rectangle: &self, comparison_triangle: Rectangle) -> bool {
//         if Self::calc_area(curr_rectangle.width, curr_rectangle.height) > calc_area(comparison_triangle.width, comparison_triangle.height){
//             return true
//         } else {
//             return false
//         }
//     }
// }

// fn main(){
//     let mut rec = new Rectangle();
//     rec.width = 20;
//     rec.height = 15;

//     println!("{}", rec.calc_area());
// }

// Enums & Pattern Matching

// Define an enum Shape with variants Circle(f64), Rectangle(f64, f64), Triangle(f64, f64, f64).

// enum Shape {
//     Circle(f64),
//     Rectangle {
//         a: f64,
//         b: f64,
//     },
//     Triangle {
//         a: f64,
//         b: f64,
//         c: f64,
//     },
// }

// fn find_perimiter(shape: Shape) -> f64 {
//     match shape {
//         Shape::Circle(circumference) => circumference,
//         Shape::Rectangle { a, b } => {a+b},
//         Shape::Triangle { a, b, c } => {a+b+c},
//     }
// }
// // this is the most beautiful code i have ever seen...

// fn main() {
//     let circle: f64 = find_perimiter(Shape::Circle((32.0)));
//     let rectangle: f64 = find_perimiter(Shape::Rectangle { a: (1.0), b: (2.0) });
//     let triangle: f64 = find_perimiter(Shape::Triangle { a: (1.0), b: (2.0), c: (3.0) });
//     println!("{circle}, {rectangle}, {triangle}");
// }
// Write a function that takes a Shape and returns its perimeter.
// Goal: Practice pattern matching.

// Intermediate Exercises

// Ownership Challenge
// Write a function that takes a Vec<String> and removes duplicates.
// Return the deduplicated vector.
// Goal: Practice ownership, HashSet, and iterators.
// use std::collections::HashSet;

// fn remove_duplicates(input: Vec<String>) -> HashSet<String>{
//     let mut output:HashSet<String> = HashSet::<String>::new();

//     for i in input {
//         output.insert(i.to_string());
//     }

//     output
// }

// fn main(){
//     let input: Vec<String> = ["hello", "hello", "yellow"]
//     .iter()
//     .map(|s| s.to_string())
//     .collect();

//     let output = remove_duplicates(input);

//     println!("{:?}", output);
// }

// Error Handling
// Read a text file line by line and convert each line to an integer.
// Handle potential errors using Result and ?.
// Goal: Practice error propagation.
// use std::{ fmt::format, fs };
// use std::io;

// fn read_content(path: &str) -> Result<String, io::Error> { // for fs ops you need io::Error in Result enum
//     let contents = fs::read_to_string(&path);
//     return contents;
// }

// fn main() {
//     let path = "error_handling.txt";
//     let contents = read_content(&path).expect("Couldn't read from file");
//     let text = contents.chars();
//     println!("Before:\n{contents}");

//     let mut new_write: String = "".to_string();
//     let mut iterator: u16 = 1;

//     for char in text {
//         // need to slice line up until \n is found.
//         if char == '\n' {
//             let mut out: String = iterator.to_string();
//             out.push_str("\n");

//             new_write.push_str(&out); // adds i as int to the line but as strnig
//             iterator += 1;
//         }
//     }

//     let _ = fs::write(&path, new_write).expect("Could not write to file");

//     let contents = read_content(&path).expect("Couldn't read from file");

//     println!("After:\n{contents}");
// }

// Iterators and Closures
// Given a vector of integers, return a new vector containing only the squares of even numbers.
// Implement this using iterator methods (filter, map, collect).
// Goal: Practice idiomatic Rust functional programming.

// fn sqrt(x: i32) -> i32 {
//     (x as f64).sqrt() as i32
// }

// fn main() {
//     let nums: Vec<i32> = [1, 2, 3, 4, 5, 6, 7, 8, 64].to_vec();
//     // personally the for loop felt cleaner...
//     // for n in nums {
//     //     if n % 2 == 0 {
//     //         let sqrt_n: i32 = sqrt(n as f64);
//     //         if sqrt_n.pow(2) == n {
//     //             output.push(n);
//     //         }
//     //     }
//     // }

//     let output: Vec<i32> = nums
//         .iter()
//         .filter_map(|&n| {// filter_map requires Option!!!
//             if n % 2 == 0 && sqrt(n).pow(2) == n {
//                 Some(n)
//             } else {
//                 None
//             }
//         })
//         .collect();

//     println!("{:?}", output);
// }

// Lifetime Exercise
// Write a function longest<'a>(x: &'a str, y: &'a str) -> &'a str that returns the longer string.
// Test with various string references.
// Goal: Understand lifetimes in function signatures.

// Advanced Exercises

// Trait Implementation
// Define a trait Summary with a method summarize.
// Implement it for two structs, e.g., NewsArticle and Tweet.
// Goal: Practice traits, generics, and polymorphism.

// Concurrency
// Use std::thread to spawn multiple threads, each calculating the sum of a slice of a large vector.
// Merge the results safely using Arc<Mutex<Vec<i32>>>.
// Goal: Learn safe concurrency with Rust.

// so one vec. two threads. each thread has half of the vec. then return the total of both halfs at end of process...

// use std::thread;
// use std::time::Duration;
// use std::sync::{ Arc, Mutex };

// Rust concurrency safety
// mutex makes shared data safe to access across multiple threads by ensuring that only one thread can access the data at a time.
// Arc (Atomic Reference Counted) allows multiple threads to share ownership of the same data, and it automatically deallocates the data when there are no more references to it.
// By combining Arc and Mutex, we can safely share and modify data across multiple threads without risking data corruption

// fn main() {
//     let input = Arc::new((1..=100).collect::<Vec<i32>>());

//     let input1 = Arc::clone(&input);
//     let input2 = Arc::clone(&input);

//     let total = Arc::new(Mutex::new(0));
//     let total1 = Arc::clone(&total);
//     let total2 = Arc::clone(&total);

//     let first_half = thread::spawn(move || {
//         for i in input1.iter().take(50) {
//             *total1.lock().unwrap() += *i as u32;
//             thread::sleep(Duration::from_millis(10));
//         }
//         total1;
//     });

//     let second_half = thread::spawn(move || {
//         for i in input2.iter().skip(50) {
//             *total2.lock().unwrap() += *i as u32;
//             thread::sleep(Duration::from_millis(10));
//         }
//         total2;
//     });

//     first_half.join().expect("first half panicked and died visciously lol");
//     second_half.join().expect("second half panicked and died visciously lol");
//     let total = total.lock().expect("total mutex got poisoned by a thread panicking");
//     println!("Total: {}", *total);
// }



#[allow(dead_code)]
fn count_truths(input: Vec<bool>) -> u16 {
    let truths = input.iter().filter(|i| **i==true).count() as u16;
    println!("found truths: {truths}");
    return truths;
}

// Write a function that takes in a string and returns a function that returns it.
#[allow(dead_code)]
fn redundant(input: String) -> impl Fn() -> String {
    move || input.to_string()
}

// Create a function that takes a variable number of arguments, each argument representing the number of items in a group.
// The function should return the number of permutations (combinations) of choices you would have if you selected one item from each group.
#[allow(dead_code)]
fn combinations(input: Vec<i16>) -> i16 {
     input.iter().fold(1, |acc, x| acc * x) // just like reduce in ts/js
}

// How many toy cars can you build?
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

// Return a vector with the sum of even numbers first and odd numbers second.
#[allow(dead_code)]
fn total_nums(input: Vec<i16>) -> Vec<i16> {
    let odd_total = input.iter().filter(|&x| {x % 2 == 0}).sum();
    let even_total = input.iter().filter(|&x| {x % 2 == 1}).sum();
    [odd_total, even_total].to_vec()
}

// uncensor("Wh*r* d*d my v*w*ls g*?", "eeioeo") ➞ "Where did my vowels go?"
#[allow(dead_code)]
fn uncensor(phrase: String, missing_vowels: String) -> String {
   let mut i: usize = 0;
    let output = phrase.chars().map(|c| {
        if c == '*' {
            let vowel = missing_vowels.chars().nth(i).expect("not enough vowels provided");
            i += 1;
            vowel
        } else {
            c
        }
    }).collect::<String>().to_string();
    output
}

// Additional practice challenges

// Reverse the order of words in a sentence.
#[allow(dead_code, unused_variables)]
fn reverse_words(sentence: String) -> String {
    let output = sentence.split_whitespace().rev().collect::<Vec<&str>>().join(" ");
    // this feels like sentence.split(" ").reverse().join(" ") in js but its not because split_whitespace is better than split(" ") since it handles multiple spaces and tabs and stuff. also rev() is reverse and collect turns it into a vec and join turns it back into a string with spaces in between.
    output
}

// Multiply all numbers in a vector by the given factor.
#[allow(dead_code, unused_variables)]
pub fn multiply_elements(values: Vec<i32>, factor: i32) -> Vec<i32> {
    values.iter().map(|x| x * factor).collect::<Vec<i32>>()
}

// Return the maximum value in a vector, or `None` if the vector is empty.
#[allow(dead_code, unused_variables)]
pub fn find_max(values: Vec<i32>) -> Option<i32> {
    values.iter().max().cloned() // max returns an Option<&i32> so need to clone it to get an Option<i32>
}

fn remove_whitespace(s: &mut String) {
    s.retain(|c| !c.is_whitespace());
    // The .retain method takes a closure (a function-like block) and evaluates every element in a collection.
    // It keeps the elements where the condition is true and deletes/drops the elements where it evaluates to false
}

// Check whether a string is a palindrome, ignoring spaces and case.
#[allow(dead_code, unused_variables)]
pub fn is_palindrome(text: &str) -> bool {
    let mut normalized_text : String = text.to_lowercase();
    remove_whitespace(&mut normalized_text);

    let palindrome_check :bool = normalized_text == normalized_text.chars().rev().collect::<String>();
    palindrome_check
}

// Sum the digits of a non-negative number.
// example: sum_digits(123) -> 6 (1 + 2 + 3)
#[allow(dead_code, unused_variables)]
pub fn sum_digits(number: u32) -> u32 {
    let out = number.to_string().split("").map(|c| c.parse::<u32>().unwrap_or(0)).sum::<u32>();
    out
}
