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

enum Shape {
    Circle(f64),
    Rectangle {
        a: f64,
        b: f64,
    },
    Triangle {
        a: f64,
        b: f64,
        c: f64,
    },
}

fn find_perimiter(shape: Shape) -> f64 {
    match shape {
        Shape::Circle(circumference) => circumference,
        Shape::Rectangle { a, b } => {a+b},
        Shape::Triangle { a, b, c } => {a+b+c},
    }
}
// this is the most beautiful code i have ever seen...

fn main() { 
    let circle: f64 = find_perimiter(Shape::Circle((32.0)));
    let rectangle: f64 = find_perimiter(Shape::Rectangle { a: (1.0), b: (2.0) });
    let triangle: f64 = find_perimiter(Shape::Triangle { a: (1.0), b: (2.0), c: (3.0) });
    println!("{circle}, {rectangle}, {triangle}");
}
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
