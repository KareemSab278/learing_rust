mod challenges;
mod neet_code;

fn main() {
    let nums: Vec<i32> = vec![2, 7, 11, 15];
    println!("{:?}", neet_code::product_except_self(nums));
}
