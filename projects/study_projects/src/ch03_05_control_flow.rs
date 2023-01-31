pub fn main() {
    // controle_if();
    // controle_if_in_let();
    // iterate_loop();
    // iterate_while();
    // iterate_for();
    iterate_range();
}

fn controle_if() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    if number != 0 {
        println!("number was something other than zero");
    }

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}

fn controle_if_in_let() {
    let condition = false;
    let number = if condition { 5 } else { 6 };

    // let wrong = if condition { 5 } else { "q" };

    println!("The value of number is: {}", number);
}

fn iterate_loop() {
    loop {
        // println!("again!");
    }
}

fn iterate_while() {
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }

    println!("LIFTOFF!!!");
}

fn iterate_for() {
    let a = [10, 20, 30, 40, 50];
    // let mut index = 0;

    // while index < 5 {
    //     println!("the value is: {}", a[index]);

    //     index += 1;
    // }

    for element in a.iter() {
        println!("the value is: {}", element);
    }
}

fn iterate_range() {
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}
