pub fn copy_range<T>(v: &Vec<T>, start: usize, len: usize) -> Vec<T>
where
    T: Clone,
{
    let mut copy = Vec::with_capacity(len);
    for i in start..start + len {
        copy.push(v[i].clone());
    }
    return copy;
}

#[allow(unused)]
pub fn print_2d_vec<T>(v: &Vec<Vec<T>>)
where
    T: ToString,
{
    for line in v {
        let mut st = String::new();
        for t in line {
            st.push_str(t.to_string().as_str());
        }
        println!("{}", st);
    }
}

#[allow(unused)]
pub fn print_2d_vec_bool(v: &Vec<Vec<bool>>, t: char, f: char) {
    for line in v {
        let mut st = String::new();
        for b in line {
            st.push(if *b { t } else { f });
        }
        println!("{}", st);
    }
}
