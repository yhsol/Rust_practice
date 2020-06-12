fn main() {
    let mut v = vec![1, 2, 3];
    for i in &mut v {
        *i += 50;
        // println!("{}", i)
    }

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = format!("{}{}", s1, s2);

    let hello = "hello world!";

    // println!("{}", hello.len());
    let s = &hello[0..1];
    // println!("{}", s)

    // hashmap

    use std::collections::HashMap;
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
}
