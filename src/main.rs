// Rust Primitive Data Types Example
// ----------------------------------
// Integers: i32 (signed 32-bit), u64 (unsigned 64-bit), etc.
// Floating point: f32 (32-bit), f64 (64-bit)
// Boolean: bool (true or false)
// Character: char (Unicode scalar value)
fn add(a: i32, b: i32) -> i32 {
    a + b
} // is this a lambda?

fn main() {
    // --- Integer Types ---
    let x: i32 = 40; // 32-bit signed integer (can be negative or positive)
    let y: u64 = 2; // 64-bit unsigned integer (only positive values)
    println!("signed int {} and unsigned int {}", x, y);

    println!("{}", add(x, y as i32)); // apparently you can add functions in params lol

    // --- Array Type ---
    let nums: [i32; 5] = [1, 2, 3, 4, 5]; // Fixed-size array of 5 i32 values
    println!("arr is {:?}", nums);

    // --- String Slice Array ---
    let words: [&str; 3] = ["word1", "word2", "word3"]; // Array of string slices
    println!("words: {:?}", words);

    // --- Tuple Type ---
    let tuple = ("kareem", 25, true); // Tuple with a string, integer, and boolean
    println!("{:?}", tuple);
    // so the :? means something like "debuggable" format or sumn

    // --- Floating Point and Boolean Types ---
    let pi: f64 = 3.141592654; // 64-bit floating point number
    let is_decimal: bool = true; // Boolean value
    println!("pi is {} and is {}", pi, is_decimal);

    // --- Character Type ---
    let letter: char = 'a'; // Unicode character
    println!("first letter of alphabet is '{}'", letter);

    // SLICES: i32, &str, String
    let number_slice: &[i32] = &[1, 2, 3, 4, 5];
    println!("{:?}", number_slice);

    let str_slices: &[String] = &["str1".to_string()];
    println!("{:?}", str_slices);

    let mut string_example: String = String::from(
        "i am a string of String type. Allocated on the heap with dynamic memory size. I am also slower than &str"
    ).to_string();

    // you could use String::from OR .to_string(); it is just personal preference at this point bruh
    string_example.push_str(". You can use push_str() to add stuff to me lol");
    // remember, nothing in rust is mutable, youll need to add the mut keyword before the var name to ensure it can be mutated.
    let string_slice_example: &str = &string_example;
    // you can also slice stuff by using the [x..y] option. so like &string_example[0..6];
    println!("{}", string_slice_example);
    say_hello();
    return_height(5.7);
    references_in_rust();
}


// FUNCTIONS

fn say_hello() {
    println!("hello!")
}

fn return_height(height: f32) {
    println!("height is {}", height);
}

fn references_in_rust() {
    let owned = String::from("i'm owned");
    let new_owned = String::from(append_str(&owned, " but now under new_owned")); // ownership borrowed here.
    // let new_owned = owned; // this would mean i transfered ownership completely to new_owned. 
    println!("{}", owned);
    println!("{}", new_owned);
}

fn append_str(base: &str, string: &str)-> String{
    let mut output = base.to_string(); // to_string() makes a copy, no ownership changed...
    output.push_str(string);
    return output;
}
