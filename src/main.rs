// doing rust challenges to code better.

// Temperature Converter
// Convert Celsius ↔ Fahrenheit from user input.
enum TempType {
    Celsius,
    Fahrenheit,
}
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

use rand::Rng;
use read_input::prelude::*;

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
use std::collections::HashMap;

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
#[derive(Debug)]
struct ToDo {
    id: u16,
    note: String,
}

impl ToDo {
    fn add(todos: &mut Vec<ToDo>, id: u16, note: String) {
        todos.push(ToDo { note, id });
    }

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
struct Account {
    acc_id: u32,
    name: String,
    balance: f64,
}

// let max_value = arr.iter().max().unwrap();

impl Account {
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
enum Shape {
    Circle(u32),
    Rectangle(u32, u32),
    Triangle(u32, u32, u32),
}

impl Shape {
    fn calc_area(shape: Shape) {
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

fn main() {
    // apparently everything will be an error unless you have a main fn...
    let rectangle= Shape::Rectangle((20), (50));
    let _ = Shape::calc_area(rectangle);
}

// 🔴 Level 4: Error Handling & Files

// File Line Counter
// Count lines, words, characters in a file.
// Learn: Result, ?, file I/O.

// already working on this in another project
// done with ts

// Config Parser (Tiny)
// Parse a file like:
