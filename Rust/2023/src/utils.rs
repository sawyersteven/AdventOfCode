pub fn split_into(raw_input: String, dest: &mut Vec<String>) {
    for s in raw_input.lines() {
        dest.push(String::from(s));
    }
}
