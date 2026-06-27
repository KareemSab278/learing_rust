mod challenges;
mod neet_code;

fn main() {
    let word = "hello";
    let word1 = "olleh";
    println!("{}", neet_code::valid_anagram(word, word1));
}
