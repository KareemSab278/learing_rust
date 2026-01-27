// doing rust challenges to code better.

// Temperature Converter
// Convert Celsius ↔ Fahrenheit from user input.

// enum TempType{
//     Celsius,
//     Fahrenheit
// }
// fn convert_temp(input: f32, tmp_type: TempType) -> f32 {
//     // could have used char but char is always 4 bytes. str is 1 byte but can be bigger. for simple letter its 1b.
//     // let input = input as f64; // i had no idea you coul do this....
//     match tmp_type{
//         TempType::Celsius => return (input * 1.8) + 32.0,
//         TempType::Fahrenheit => return (input - 32.0) / 1.8,
//     }
// }
// fn main() {
//     let temp_to_f = convert_temp(0.0, TempType::Celsius);
//     let temp_to_c = convert_temp(212.4, TempType::Fahrenheit);
//     println!("{}", temp_to_f);
//     println!("{}", temp_to_c);
// }

// ===

// NEXT:
// Number Guessing Game
// Computer picks a number, user guesses until correct.
// Learn: loops, match, randomness.













// Level 1: Basics

// Temperature Converter
// Convert Celsius ↔ Fahrenheit from user input.
// Learn: variables, input, parsing, functions.

// Number Guessing Game
// Computer picks a number, user guesses until correct.
// Learn: loops, match, randomness.

// FizzBuzz (Rust-style)
// Print numbers 1–100 with Fizz/Buzz rules.
// Learn: conditionals, loops.

// 🟡 Level 2: Ownership & Collections

// Word Counter
// Read a sentence and count how many times each word appears.
// Learn: HashMap, borrowing, lifetimes (light).

// Reverse a String (Safely)
// Reverse a UTF-8 string correctly (no .chars().rev().collect() shortcut at first 😉).
// Learn: strings, iterators, Unicode.

// Todo List (CLI)
// Add, remove, and list tasks.
// Learn: structs, Vec, mutability.

// 🟠 Level 3: Structs & Enums

// Simple Bank Account
// Deposit, withdraw, prevent overdrafts.
// Learn: methods, impl, error handling.

// Shape Area Calculator
// Enum for Circle, Rectangle, Triangle.
// Learn: enums, pattern matching.

// 🔴 Level 4: Error Handling & Files

// File Line Counter
// Count lines, words, characters in a file.
// Learn: Result, ?, file I/O.

// Config Parser (Tiny)
// Parse a file like: