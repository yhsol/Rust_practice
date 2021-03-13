fn main() {
    using_if_in_let();
}

fn else_if() {
    let number = 6;

    if number % 4 == 0 {
        return println!("number is divisible by 4");
    }
    if number % 3 == 0 {
        return println!("number is divisible by 3");
    }
    if number % 2 == 0 {
        return println!("number is divisible by 2");
    } else {
        return println!("number is not divisible by 4, 3, or 2");
    }
}

fn using_if_in_let() {
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {}", number);
}
