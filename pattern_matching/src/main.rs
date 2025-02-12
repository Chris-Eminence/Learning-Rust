

// match target {
//     Pattern1 => Expression1,
//     Pattern2 => {
//         Statement1;
//         Statement2;
//         Expression2
//     },
//     _ => Expression3
// }

enum Blockchain{
    Bitcoin,
    Etherium,
    Starknet,
    Solana,
}
fn main() {
    second_main();
    
    let block_chain = Blockchain:: Solana;

    match block_chain{
        Blockchain::Bitcoin => println!("Bitcoin"),
        Blockchain::Etherium |Blockchain :: Starknet=> println!("Etherium or Starknet"),
        _ => println!("Other Blockchains or just Solana"),

    }
}

enum Shape {
    Circle(f64),
    Rectangle(f64, f64),
    Square(f64),
}

fn calculate_area(shape: &Shape) -> f64 {
    match shape {
        // Get the bound value from the matching pattern, such as radiux, width, side
        Shape::Circle(radius) => std::f64::consts::PI * radius * radius,
        Shape::Rectangle(width, height) => width * height,
        Shape::Square(side) => side * side,
    }
}
struct Point {
    x: i32,
    y:i32,
}

fn process_point(point: Point) {
    match point {
        Point { x: 0, y: 0 } => println!("Coordinates are at the origin"),
        Point { x, y } => println!("Coordinates are at ({}, {})", x, y),
    }
}

fn second_main() {
    let circle = Shape::Circle(3.0);
    let rectangle = Shape::Rectangle(4.0, 5.0);
    let square = Shape::Square(2.0);

    // 1. Call the function to output the area of each shape
    println!("The area of a circle: {}", calculate_area(&circle));
    println!("The area of the rectangle: {}", calculate_area(&rectangle));
    println!("Area of square: {}", calculate_area(&square));

    // 2. Match pattern matching for assignment
    let area = match circle {
        Shape::Circle(radius) => std::f64::consts::PI * radius * radius,
        Shape::Rectangle(width, height) => width * height,
        Shape::Square(side) => side * side,
    };
    println!("The area of the circle: {}", area);

    // 3. Deconstruct the structure
    let point1 = Point { x: 0, y: 0 };
    let point2 = Point { x: 3, y: 7 };
    process_point(point1);
    process_point(point2);

        // 4. if let simple matching
    let some_u8_value = Some(3u8);
    match some_u8_value {
        Some(3) => println!("three"),
        // Other values besides 3 and None values should also be considered here
        _ => (),
    }
   
    //Only match the value 3
    if let Some(3) = some_u8_value {
        println!("three");
    }
}
