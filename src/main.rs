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

use std::thread;
use std::time::Duration;
use std::sync::{ Arc, Mutex };

// Rust concurrency safety
// mutex makes shared data safe to access across multiple threads by ensuring that only one thread can access the data at a time.
// Arc (Atomic Reference Counted) allows multiple threads to share ownership of the same data, and it automatically deallocates the data when there are no more references to it.
// By combining Arc and Mutex, we can safely share and modify data across multiple threads without risking data corruption

fn main() {
    let input = Arc::new((1..=100).collect::<Vec<i32>>());

    let input1 = Arc::clone(&input);
    let input2 = Arc::clone(&input);

    let total = Arc::new(Mutex::new(0));
    let total1 = Arc::clone(&total);
    let total2 = Arc::clone(&total);

    let first_half = thread::spawn(move || {
        for i in input1.iter().take(50) {
            *total1.lock().unwrap() += *i as u32;
            thread::sleep(Duration::from_millis(10));
        }
        total1;
    });

    let second_half = thread::spawn(move || {
        for i in input2.iter().skip(50) {
            *total2.lock().unwrap() += *i as u32;
            thread::sleep(Duration::from_millis(10));
        }
        total2;
    });

    first_half.join().expect("first half panicked and died visciously lol");
    second_half.join().expect("second half panicked and died visciously lol");
    let total = total.lock().expect("total mutex got poisoned by a thread panicking");
    println!("Total: {}", *total);
}
