use std::io::{stdin, stdout, Write};
pub fn split_input(raw_input: String) -> Vec<String> {
    return raw_input.split("\n").map(|x| String::from(x)).collect();
}

pub fn read_user_input(prompt: &str) {
    println!("{prompt}");
    let mut s = String::new();
    let _ = stdout().flush();
    stdin().read_line(&mut s).expect("Did not enter a correct string");
}
