mod challenges;
mod neet_code;

fn main() {
    let nums = vec![1, 2, 3, 4, 5, 1];
    println!("{}", neet_code::has_duplicate(nums));
}
