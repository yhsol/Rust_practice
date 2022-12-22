fn string_to_number() {
    let _guess: u32 = "42".parse().expect("Not a number!");
}

fn f() {
    let _x = 2.0;

    let _y: f32 = 3.0;
}

fn calc() {
    let _sum = 5 + 10;

    let _difference = 95.5 - 4.3;

    let _product = 4 * 3;

    let _quotient = 56.7 / 32.2;

    let _remainder = 43 % 5;
}

fn bool() {
    let _t = true;

    let _f: bool = false;
}

fn char() {
    let _c = 'z';
    let _z = 'ℤ';
    let _heart_eyed_cat = '😻';
}

fn tup() {
    let tup = (500, 6.4, 1);

    let (_x, y, _z) = tup;

    println!("The value of y is: {}", y);
}

fn tup_index() {
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let _five_hundred = x.0;

    let _six_point_four = x.1;

    let _one = x.2;
}

fn arr() {
    let a = [1, 2, 3, 4, 5];

    let _first = a[0];
    let _second = a[1];

    let index = 10;

    let element = a[index];

    println!("The value of element is: {}", element);
    //=> panicked by index out of bounds
}

fn main() {
    string_to_number();
    f();
    calc();
    bool();
    char();
    tup();
    tup_index();
    arr();
}
