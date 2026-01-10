// RUST LOOPS — JS DEV CHEAT SHEET

fn main() {
    let mut condition: i8 = 10;

    // loop { // basic "loop" (ill probably use this more often)
    //     println!("current = {condition}");
    //     if condition == 0 {break};
    //     condition-=1;
    // }


    // ===

    let mut new_vector: Vec<i32> = Vec::new();
    for mut _i in 0..=10 { // "for in" loop
        new_vector.push(_i+1);
    }
    println!("{:?}", new_vector);

    // ===

    while condition > 0 { // "while" loop
        println!("condition count: {condition}");
        condition -=1;
    }

}


