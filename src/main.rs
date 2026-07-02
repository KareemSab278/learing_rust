mod challenges;
mod neet_code;

fn main() {
    
    println!("{:?}", neet_code::Solution::encode(vec!["Hello".to_string(), "World".to_string()]));
    println!("{:?}", neet_code::Solution::decode("Hello@^^^@World".to_string()));
}
