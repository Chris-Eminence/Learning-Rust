// program to calculate the area of a rectangle

struct Rectangle {
    length: u32,
    width: u32,
}

fn main() {
    let rect1 = Rectangle {
        length: 50,
        width: 30,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1),
    );
}

pub fn area(rectangle: &Rectangle) -> u32 {
    rectangle.length * rectangle.width
}