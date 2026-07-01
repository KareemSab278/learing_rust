mod challenges;
mod neet_code;

fn main() {
    let nums : Vec<i32> = Vec::from([1,2,1,1,1,3,4,4,4]);
    println!("{:?}", neet_code::top_k_frequent(nums, 2));
}
