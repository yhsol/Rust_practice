fn main() {
    // loop_fn();
    returning_values_from_loops();
}

fn loop_fn() {
    loop {
        println!("again!");
    }
}

fn returning_values_from_loops() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("Ther result is {}", result);
}
