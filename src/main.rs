mod challenges;
mod neet_code;

fn main() {
    let strs : Vec<String> = Vec::from(["act","pots","tops","cat","stop","hat"]).iter().map(|s| s.to_string()).collect();
    println!("{:?}", neet_code::group_anagrams(strs));
}
