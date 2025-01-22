fn main() {
    // Calls each function sequentially
    f1();
    f2();
    f3();
    f4();
    f5();
    f6();
    f7();
    f8();
}

// Function demonstrating an if-else condition
fn f1() {
    let x = 4;

    if x < 4 {
        // Executes if x is less than 4
        println!("This runs because x is less");
    } else {
        // Executes if x is greater than or equal to 4
        println!("This runs because x is greater");
    }
}

// Function demonstrating multiple conditions using else if
fn f2() {
    let number = 6;

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

// Function demonstrating conditional assignment in a let statement
fn f3() {
    let condition = false;

    // Assigns either 5 or 6 based on the value of condition
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");
}

// Function demonstrating a loop with a return value
fn f4() {
    let mut counter = 0;

    // Infinite loop that breaks when counter reaches 10
    let result = loop {
        counter += 1;
        println!("counter is {counter} at this iteration");

        if counter == 10 {
            // Breaks the loop and returns counter * 2
            break counter * 2;
        }
    };

    println!("The result is {result}");
}

// Function demonstrating labeled loops and breaking from an outer loop
fn f5() {
    let mut count = 0;

    // Labeled outer loop
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");

            if remaining == 9 {
                // Breaks the inner loop
                break;
            }
            if count == 2 {
                // Breaks the outer loop
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }

    println!("End count = {count}");
}

// Function demonstrating a while loop
fn f6() {
    let mut number = 3;

    // Loops while number is not 0
    while number != 0 {
        println!("{number}!");
        number -= 1;
    }

    println!("LIFTOFF!!!");
}

// Function demonstrating a while loop to iterate through an array
fn f7() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    // Iterates through the array using a while loop
    while index < 5 {
        println!("the value is: {}", a[index]);
        index += 1;
    }
}

// Function demonstrating a for loop to iterate through an array
fn f8() {
    let a = [10, 20, 30, 40, 50];

    // Iterates through the array using a for loop
    for element in a {
        println!("the value is: {element}");
    }
}
