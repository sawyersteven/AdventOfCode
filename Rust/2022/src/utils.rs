pub fn split_into(raw_input: String, dest: &mut Vec<String>) {
    for s in raw_input.split("\n") {
        dest.push(String::from(s));
    }
}

pub fn lcm(a: usize, b: usize) -> usize {
    let (mut x, mut y) = (a.min(b), a.max(b));
    let mut rem = x % y;
    while rem != 0 {
        x = y;
        y = rem;
        rem = x % y;
    }
    return (a * b) / y;
}
