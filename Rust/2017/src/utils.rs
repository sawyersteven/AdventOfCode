pub fn split_into(raw_input: String, dest: &mut Vec<String>) {
    for s in raw_input.split("\n") {
        dest.push(String::from(s));
    }
}

pub fn get_input() -> String {
    use std::io::{stdin, stdout, Write};
    let mut s = String::new();
    print!("Enter to continue...");
    let _ = stdout().flush();
    stdin().read_line(&mut s).expect("Did not enter a correct string");
    if let Some('\n') = s.chars().next_back() {
        s.pop();
    }
    if let Some('\r') = s.chars().next_back() {
        s.pop();
    }
    return s;
}

pub fn vec2d<T>(default_val: T, len: usize, width: usize) -> Vec<Vec<T>>
where
    T: Clone,
{
    let mut vec = Vec::<Vec<T>>::with_capacity(len);
    for _ in 0..len {
        let mut row = Vec::<T>::with_capacity(width);
        for _ in 0..width {
            row.push(default_val.clone());
        }
        vec.push(row);
    }

    return vec;
}
