fn main() {
    println!("The value of x is {} ", fibo(10));
}

// fn another_function(x: i32, y: i32) {
//     println!("The value of x is: {}", x);
//     println!("The value of y is: {}", y);
// }

fn fibo(n: u32) -> u32 {
    // if n <= 1 {
    //     return n;
    // } else {
    //     fibo(n - 1) + fibo(n - 2)
    // }

    // if n == 0 {
    //     return 0;
    // } else if n == 1 {
    //     return 1;
    // }

    // fibo(n - 1) + fibo(n - 2)

    match n {
        0 => 0,
        1 => 1,
        _ => fibo(n - 2) + fibo(n - 1),
    }
}
