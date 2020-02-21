fn main() {
    let mut s1 = String::from("hello");
    // let len = calculate_length(&mut s1);
    // println!("s1: {}, len: {}", s1, len);
    // change(&mut s1);
    {
        let r1 = &mut s1;
    }
    let r2 = &mut s1;
    println!("{}, {}", r1, r2)
}

fn change(s: &mut String) {
    s.push_str(", world");
    println!("{}", s)
}

fn calculate_length(s: &mut String) -> usize {
    s.len()
}
