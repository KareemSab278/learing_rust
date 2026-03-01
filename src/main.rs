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

fn sum_vec(input: Vec<i32>)-> i32{
    let mut tot: i32 = 0;
    for num in input{
        tot += num
    }
    tot
}

fn main(){
    let nums: Vec<i32> = [1,1,1,1,1,1].to_vec();
    let output: i32 = nums.iter().sum();
    println!("{output}"); 
}

// Structs & Methods

// Define a struct called Rectangle with width and height.

// Implement a method area that returns the area.

// Implement another method can_hold(&self, other: &Rectangle) that returns true if self can fully contain other.

// Enums & Pattern Matching

// Define an enum Shape with variants Circle(f64), Rectangle(f64, f64), Triangle(f64, f64, f64).

// Write a function that takes a Shape and returns its perimeter.

// Goal: Practice pattern matching.

// Intermediate Exercises

// Ownership Challenge

// Write a function that takes a Vec<String> and removes duplicates.

// Return the deduplicated vector.

// Goal: Practice ownership, HashSet, and iterators.

// Error Handling

// Read a text file line by line and convert each line to an integer.

// Handle potential errors using Result and ?.

// Goal: Practice error propagation.

// Iterators and Closures

// Given a vector of integers, return a new vector containing only the squares of even numbers.

// Implement this using iterator methods (filter, map, collect).

// Goal: Practice idiomatic Rust functional programming.

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

// Smart Pointers

// Implement a simple linked list using Box.

// Add methods to append, remove, and print nodes.

// Goal: Understand heap allocation and recursive types.

// Macros

// Write a macro vec_of_strings! that creates a Vec<String> from a list of literals.

// Goal: Learn how to write simple declarative macros.