pub fn split_into(raw_input: String, dest: &mut Vec<String>) {
    for s in raw_input.split("\n") {
        dest.push(String::from(s));
    }
}

pub fn gcd(a: isize, b: isize) -> isize {
    let mut b = b;
    let mut a = a;
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    return a.abs();
}

pub fn lcm(nums: &[isize]) -> isize {
    let mut answer = (nums[0] * nums[1]) / gcd(nums[0], nums[1]);

    for i in 2..nums.len() {
        answer = (answer * nums[i]) / gcd(answer, nums[i]);
    }
    return answer;
}
