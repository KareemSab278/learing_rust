// mod memory_types_fn_vars; // module
// use memory_types_fn_vars::one; // :: means path access in rust

fn main() {
    // one();
    let mut demo = Example {
        text: String::from("i am text"), // if you get "expected `String`, found `&str`" error
        number: 28,
    };
    demo.increment_num();
    demo.read_num();
    demo.return_text();

    // what if i had a const var and tried taking it's value and manioulating it? hmmm

    const VARIABLE: i8 = 2;
    // YOU CANNOT DO BORROW REFERENCE TO VARIABLE BECAUSE VARIABLE ITSELF CANNOT BE MUTABLE TYPE
    // THUS, YOU MUST REASSIGN AND THEN USE IN MUTABLE VARIABLE
    let mut variable = VARIABLE; // i am now giving ownership to variable.
    variable += 1;

    // IF VARIABLE WAS ORIGINALLY A MUTABLE TYPE USING LET, YOU CAN BORROW REF FROM &MUT AND MANIPULATE THROUGH & LIKE:
    let mut owner = String::from("owner");
    let borrower = &mut owner; // HERE I AM MANIPULATING OWNER ONLY. BORROWER CAN ONLY HAVE ACCESS TO OWNER HERE
    // YOU CAN ONLY USE BORROWER FROM HERE ON UNLESS:

    // let mut owner = String::from("owner");
    // {
    //     let borrower = &mut owner;
    //     // use borrower here
    // } // borrower goes out of scope here
    // // now you can use owner again
    // println!("{}", owner);

    println!("VARIABLE: {}, variable: {}", VARIABLE, variable);

    // SHADOWING:
    // In summary: shadowing is a way to transform or reuse names, but it creates a new variable, not just updates the old one.
    // For efficient memory use, mutation or references are better if you don’t need a new variable.
}
// structs in rust
struct Example {
    text: String,
    number: i32,
}

impl Example {
    // impl = implementation for the struct you created. kinda like adding medthods to a class in JS
    // so just an objects methods but outside the obj
    fn return_text(&self) {
        // cannot have a non mutable reference if you plan to mutate it. We're only reading so it is fine.
        println!("text: {}", self.text)
    }

    fn increment_num(&mut self) {
        self.number += 1;
    }

    fn read_num(&self) {
        println!("num: {}", self.number);
    }
}
