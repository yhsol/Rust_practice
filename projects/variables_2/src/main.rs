const MAX_POINTS: u32 = 100_000;

fn shadow() {
    let x = 5;

    let x = x + 1;

    let x = x * 2;

    println!("The value of x is in shadow: {}", x);

    // shadowing
    let spaces = "    ";
    let spaces = spaces.len();
    println!("The value of spaces: {}", spaces);

    // mut
    let spaces_mut = "    ";
    spaces_mut = spaces_mut.len();
    println!("The value of spaces_mut: {}", spaces_mut);
}

fn main() {
    // immutable
    // let x = 5;
    // println!("The value of x is immutable: {}", x);
    // x = 6;
    // println!("The value of x is immutable: {}", x);

    // mutable
    // let mut x = 5;
    // println!("The value of x is mutable: {}", x);
    // x = 6;
    // println!("The value of x is mutable: {}", x);

    shadow();
}
