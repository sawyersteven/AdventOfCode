use std::io;

pub fn split_into(raw_input: String, dest: &mut Vec<String>) {
    for s in raw_input.split('\n') {
        dest.push(String::from(s));
    }
}

pub fn get_input() -> String {
    let mut buffer = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut buffer).unwrap();
    return buffer;
}
