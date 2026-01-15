enum Option<DataType> { // the type. if exists then cool
    Some(DataType), // exists
    None, // does not exist bruv
}

// enum Result<T, E> {
//     Ok(T),
//     Err(E), // the type of err
// }

// fn is_divisible_by_2(dividee: i32) -> Result<String, String> {
//     if dividee % 2 == 0 {
//         return Result::Ok("Divisible by 2".to_string());
//     }
//     return Result::Err("Not divisible by 2".to_string());
// }

fn age_verification(age: i32) -> Option<i32> {
    if age >= 18 {
        return Option::Some(age);
    }
    return Option::None;
}

fn main() {
    match age_verification(22) {
        Option::Some(val) => println!("AGE OK: {}", val),
        Option::None => println!("underage"),
    }
}
