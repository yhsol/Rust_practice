fn main() {
    calc_struct()
}

fn calc() {
    let length1 = 50;
    let width1 = 30;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(length1, width1)
    );
}

fn area(length: u32, width: u32) -> u32 {
    length * width
}

fn calc_tuple() {
    let rect1 = (50, 30);

    println!(
        "The area of the rectangle is {} square pixels.",
        area_tuple(rect1)
    );
}

fn area_tuple(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

struct Rectangle {
    length: u32,
    width: u32,
}

fn calc_struct() {
    let rect1 = Rectangle {
        length: 50,
        width: 30,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area_struct(&rect1)
    );
}

fn area_struct(rectangle: &Rectangle) -> u32 {
    rectangle.length * rectangle.width
}
