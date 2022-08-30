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

pub fn new_2d_vec<T>(rows: usize, columns: usize, default: T) -> Vec<Vec<T>>
where
    T: Clone,
{
    let mut outer = Vec::<Vec<T>>::with_capacity(rows);
    for _ in 0..rows {
        let mut inner = Vec::<T>::with_capacity(columns);
        for _ in 0..columns {
            inner.push(default.clone());
        }
        outer.push(inner);
    }
    return outer;
}

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

pub fn print_2d_vec_bool(v: &Vec<Vec<bool>>, t: char, f: char) {
    for line in v {
        let mut st = String::new();
        for b in line {
            st.push(if *b { t } else { f });
        }
        println!("{}", st);
    }
}
