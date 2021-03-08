fn main() {
    let x = plus_one(5);

    println!("The value of x is: {}", x);
}

fn state_and_expression() {
    let x = 5;

    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);
}

fn return_values() {
    fn five() -> i32 {
        5
    }

    let x = five();

    println!("The value of x is: {}", x);
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
