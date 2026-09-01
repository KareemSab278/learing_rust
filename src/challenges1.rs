use crate::challenges1::Shape::Circle;

/* Challenge 1: Direction enum
   Create an enum `Direction` with variants North, East, South, West.
   Implement methods to turn right, turn left, and return the compass degrees.
   Expected output:
   Direction::North.turn_right() -> East
   Direction::East.turn_left() -> North
   Direction::South.degrees() -> 180
*/
#[derive(Debug, PartialEq, Eq)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    pub fn turn_right(self) -> Result<Direction, &'static str> {
        if self == Direction::North {
            return Ok(Direction::East);
        } else if self == Direction::East {
            return Ok(Direction::South);
        } else if self == Direction::South {
            return Ok(Direction::West);
        }
        Err("Invalid direction")
    }

    pub fn turn_left(self) -> Result<Direction, &'static str> {
        if self == Direction::North {
            return Ok(Direction::West);
        } else if self == Direction::West {
            return Ok(Direction::South);
        } else if self == Direction::South {
            return Ok(Direction::East);
        }
        Err("Invalid direction")
    }

    pub fn degrees(&self) -> Result<u16, &'static str> {
        if self == &Direction::North {
            return Ok(0);
        } else if self == &Direction::East {
            return Ok(90);
        } else if self == &Direction::West {
            return Ok(270);
        } else if self == &Direction::South {
            return Ok(180);
        }
        Err("Invalid direction")
    }
}

/* Challenge 2: Rectangle struct and methods
   Define a struct `Rectangle` with width and height.
   Implement `area`, `can_hold`, and an associated function `square`.
   Expected output:
   Rectangle { width: 3, height: 4 }.area() -> 12
   Rectangle::square(5).can_hold(&Rectangle { width: 3, height: 4 }) -> true
*/
#[derive(Debug, PartialEq)]
pub struct Rectangle {
    pub width: u32,
    pub height: u32,
}

impl Rectangle {
    pub fn area(&self) -> u32 {
        self.width * self.height
    }

    pub fn can_hold(&self, other: &Rectangle) -> bool {
        Rectangle::area(self) >= Rectangle::area(other)
    }

    pub fn square(size: u32) -> Rectangle {
        Rectangle {
            width: (size),
            height: (size),
        }
    }
}

/* Challenge 3: Shape enum with area and perimeter
   Create enum `Shape` with Circle(f64), Rectangle(f64, f64), Triangle(f64, f64, f64).
   Add methods `area` and `perimeter`.
   Expected output:
   Shape::Circle(1.0).area() -> 3.14159...
   Shape::Rectangle(2.0, 3.0).perimeter() -> 10.0
*/
#[derive(Debug, PartialEq)]
pub enum Shape {
    Circle(f64),
    Rectangle(f64, f64),
    Triangle(f64, f64, f64),
}
// i suck at geometry...
impl Shape {
    pub fn area(&self) -> f64 {
        match self {
            Shape::Circle(radius) => std::f64::consts::PI * radius * radius,
            Shape::Rectangle(width, height) => width * height,
            Shape::Triangle(a, b, c) => {
                let s = (a + b + c) / 2.0;
                (s * (s - a) * (s - b) * (s - c)).sqrt()
            }
        }
    }

    pub fn perimeter(&self) -> f64 {
        match self {
            Shape::Circle(radius) => 2.0 * std::f64::consts::PI * radius,
            Shape::Rectangle(width, height) => 2.0 * (width + height),
            Shape::Triangle(a, b, c) => a + b + c,
        }
    }
}

/* Challenge 4: TrafficLight enum
   Define enum `TrafficLight` with Red, Yellow, Green.
   Implement `next` and `is_stop`.
   Expected output:
   TrafficLight::Red.next() -> Green
   TrafficLight::Yellow.is_stop() -> false
*/
#[derive(Debug, PartialEq, Eq)]
pub enum TrafficLight {
    Red,
    Yellow,
    Green,
}

impl TrafficLight {
    pub fn next(self) -> TrafficLight {
        if self == TrafficLight::Red {
            TrafficLight::Green
        } else if self == TrafficLight::Green {
            TrafficLight::Yellow
        } else {
            TrafficLight::Green
        }
    }

    pub fn is_stop(&self) -> bool {
        self == &TrafficLight::Red
    }
}

/* Challenge 5: Message enum with associated data
   Create enum `Message` with Quit, Move { x, y }, Write(String), ChangeColor(i32, i32, i32).
   Implement a method `describe` that returns a short description string.
   Expected output:
   Message::Quit.describe() -> "Quit message"
   Message::Move { x: 5, y: 7 }.describe() -> "Move to x=5, y=7"
*/
#[derive(Debug, PartialEq)]
pub enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    pub fn describe(&self) -> String {
        match self {
            Message::Quit => "Quit Message".to_string(),
            Message::Move { x, y } => format!("Move to x={x}, y={y}").to_string(),
            Message::Write(mssg) => mssg.to_string(),
            Message::ChangeColor(color1, color2, color3) => {
                format!("Changed colors to {color1}, {color2}, {color3}")
            }
        }
    }
}

/* Challenge 6: Counter struct with internal state
   Define struct `Counter` with a count field.
   Implement `new`, `increment`, and `value`.
   Expected output:
   let mut counter = Counter::new();
   counter.increment();
   counter.increment();
   counter.value() -> 2
*/
#[derive(Debug, PartialEq, Eq)]
pub struct Counter {
    count: u32,
}

impl Counter {
    pub fn new() -> Counter {
        Counter { count: 0 }
    }

    pub fn increment(&mut self) {
        self.count += 1
    }

    pub fn value(&self) -> u32 {
        self.count
    }
}

/* Challenge 7: Student struct with grade methods
   Create struct `Student` with name and scores vector.
   Implement `average` and `letter_grade`.
   Expected output:
   Student { name: "Ava" }.average() -> 90.0
   Student { name: "Ava" }.letter_grade() -> 'A'
*/
#[derive(Debug, PartialEq)]
pub struct Student {
    pub name: String,
    pub scores: Vec<u32>,
}

impl Student {
    pub fn average(&self) -> f64 {
        let sum: u32 = self.scores.iter().sum();
        let count = self.scores.len() as f64;
        if count == 0.0 {
            0.0
        } else {
            sum as f64 / count
        }
    }

    pub fn letter_grade(&self) -> char {
        // just a bunch of if elses here
        let avg = self.average();
        if avg >= 90.0 {
            'A'
        } else if avg >= 80.0 {
            'B'
        } else if avg >= 70.0 {
            'C'
        } else if avg >= 60.0 {
            'D'
        } else {
            'F'
        }
    }
}

/* Challenge 8: Currency enum with cents conversion
   Define enum `Currency` with Penny, Nickel, Dime, Quarter, Dollar(u32).
   Add `value_in_cents`.
   Expected output:
   Currency::Quarter.value_in_cents() -> 25
   Currency::Dollar(3).value_in_cents() -> 300
*/
#[derive(Debug, PartialEq, Eq)]
pub enum Currency {
    Penny,
    Nickel,
    Dime,
    Quarter,
    Dollar(u32),
}

impl Currency {
    pub fn value_in_cents(&self) -> u32 {
        match self {
            Currency::Penny => 1,
            Currency::Nickel => 5,
            Currency::Dime => 10,
            Currency::Quarter => 25,
            Currency::Dollar(amount) => amount * 100,
        }
    }
}

/* Challenge 9: Pair generic struct
   Create a generic struct `Pair<T>` with two values.
   Implement `new` and `swap`.
   Expected output:
   Pair::new(1, 2).swap() -> Pair { first: 2, second: 1 }
*/
#[derive(Debug, PartialEq)]
pub struct Pair<T> {
    pub first: T,
    pub second: T,
}

impl<T> Pair<T> {
    pub fn new(first: T, second: T) -> Pair<T> {
        Pair { first, second }
    }

    pub fn swap(self) -> Pair<T> {
        Pair {
            first: self.second,
            second: self.first,
        }
    }
}

/* Challenge 10: Status enum and matcher function
   Define enum `Status` with Ok(String), Error(String), Unknown.
   Write `describe_status` that returns a short status message.
   Expected output:
   describe_status(Status::Ok("done".to_string())) -> "Success: done"
   describe_status(Status::Error("fail".to_string())) -> "Error: fail"
*/
#[derive(Debug, PartialEq)]
pub enum Status {
    Ok(String),
    Error(String),
    Unknown,
}

pub fn describe_status(status: Status) -> String {
    // i finally understand it - it is more like a case switch which goes through every possible variant of yhe enum.
    match status {
        Status::Ok(msg) => format!("Success: {}", msg),
        Status::Error(msg) => format!("Error: {}", msg),
        Status::Unknown => "Status unknown".to_string(),
    }
}

/* Challenge 11: Async file reader
   Write an async function `read_file_contents(path: &str) -> Result<String, std::io::Error>`.
   Use async I/O utilities such as `tokio::fs::read_to_string` or `async_std::fs::read_to_string`.
   Expected output:
   read_file_contents("example.txt").await -> file contents string
*/
use std::fs::read_to_string;
pub async fn read_file_contents(path: &str) -> Result<String, std::io::Error> {
    let contents = read_to_string(path)?;
    Ok(contents)
}

/* Challenge 12: Async HTTP fetcher
   Write an async function `fetch_status(url: &str) -> Result<u16, reqwest::Error>`.
   Use an async HTTP client and return the response status code.
   Expected output:
   fetch_status("https://example.com").await -> 200
*/
pub async fn fetch_status(url: &str) -> Result<u16, reqwest::Error> {
    unimplemented!()
}

/* Challenge 13: String sanitizer
   Create a function `sanitize(input: &str) -> String` that removes punctuation and lowercases text.
   Expected output:
   sanitize("Hello, World!") -> "hello world"
*/
pub fn sanitize(input: &str) -> String {
    let keep_chars: Vec<char> = "abcdefghijklmnopqrstuvwxyz".chars().collect();

    input
        .to_lowercase()
        .chars()
        .filter(|c| keep_chars.contains(c))
        .collect::<String>()
}
/* Challenge 14: Title-case converter
   Write `title_case(input: &str) -> String` that capitalizes the first letter of each word.
   Expected output:
   title_case("rust programming") -> "Rust Programming"
*/
pub fn title_case(input: &str) -> String {
    input
        .split_whitespace() // Split by whitespace (not just spaces)
        .map(|word| {
            let mut chars = word.chars();
            if let Some(first_char) = chars.next() {
                format!("{}{}", first_char.to_uppercase(), chars.as_str())
            } else {
                String::new()
            }
        })
        .collect::<Vec<String>>()
        .join(" ")
}

/* Challenge 15: Vec filter and map
   Write `even_squares(numbers: Vec<i32>) -> Vec<i32>` that returns squares of even numbers only.
   Expected output:
   even_squares(vec![1,2,3,4]) -> vec![4, 16]
*/
pub fn even_squares(numbers: Vec<i32>) -> Vec<i32> {
    numbers
        .iter()
        .filter(|n| *n % 2 == 0)
        .map(|x| x * x)
        .collect()
}

/* Challenge 16: Vec zipper
   Write `zip_sum(a: Vec<i32>, b: Vec<i32>) -> Vec<i32>` that sums elements pairwise.
   Expected output:
   zip_sum(vec![1,2,3], vec![4,5,6]) -> vec![5,7,9]
*/
pub fn zip_sum(a: Vec<i32>, b: Vec<i32>) -> Vec<i32> {
    //     In this implementation:
    // - `a.into_iter().zip(b.into_iter())` creates an iterator that pairs elements from both vectors.
    // - `.map(|(x, y)| x + y)` applies the function to each pair of elements (summing them).
    // - `.collect()` gathers the results into a new vector.
    //. i never would have guessed it could do that wtf???
    a.into_iter()
        .zip(b.into_iter())
        .map(|(x, y)| x + y)
        .collect()
}

/* Challenge 17: HashMap frequency counter
   Create `word_count(text: &str) -> std::collections::HashMap<String, usize>`.
   Count each word occurrence in the text.
   Expected output:
   word_count("a a b") -> {"a": 2, "b": 1}
*/
use std::collections::HashMap;
pub fn word_count(text: &str) -> HashMap<String, usize> {
    text.split_whitespace()
        .into_iter()
        .fold(HashMap::new(), |mut acc, word| {
            *acc.entry(word.to_string()).or_insert(0) += 1;
            acc
        })
}

/* Challenge 18: Inventory updater
   Write `update_inventory(inventory: &mut std::collections::HashMap<String, u32>, item: &str, amount: u32)`.
   If the item exists, add the amount; otherwise insert a new entry.
   Expected behavior:
   inventory["apple"] = 3 -> update_inventory(&mut inventory, "apple", 2) -> 5
*/
pub fn update_inventory(
    inventory: &mut HashMap<String, u32>,
    item: &str,
    amount: u32,
) {
    *inventory.entry(item.to_string()).or_insert(0) += amount;
}
