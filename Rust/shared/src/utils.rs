pub fn split_input(raw_input: String) -> Vec<String> {
    return raw_input.split("\n").map(|x| String::from(x)).collect();
}
