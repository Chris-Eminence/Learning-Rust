fn main() {
    // Entry point of the program
    println!("The main function just compiled");
    
    // Calls another function without parameters
    another_function();
    
    // Calls a function with a single parameter
    function_with_parameter(34);
    
    // Calls a function with multiple parameters
    function_with_multiple_parameters(33, 'x');
    
    // Calls a function that includes a statement block
    function_with_statement();
    
    // Calls a function with a return type and captures its return value
    let s = function_with_return_type();
    println!("This is the return value from the function {s}");
    
    // Calls a function with a parameter and a return type, capturing its return value
    let a = with_return_type_and_parameters(23);
    println!("With return type and parameter is: {a}");
}

// Function without parameters
fn another_function() {
    println!("This is another function");
}

// Function with a single parameter
fn function_with_parameter(x: u32) {
    println!("This is the value of x {x}");
}

// Function with multiple parameters
fn function_with_multiple_parameters(value: i32, label: char) {
    println!("This is the label: {label}, and this is the value: {value}");
}

// Function demonstrating a statement block
fn function_with_statement() {
    let _y = 12; // A statement that declares a variable (unused in this example)

    // A statement block that calculates a value and assigns it to `n`
    let n = {
        let x = 3;
        x + 1 // The last expression in the block is returned
    };

    println!("The value of y is: {n}");
}

// Function with a return type
fn function_with_return_type() -> i32 {
    return 54; // Explicitly returns a value of type `i32`
}

// Function with a parameter and a return type
fn with_return_type_and_parameters(n: i32) -> i32 {
    return n + 5; // Adds 5 to the parameter `n` and returns the result
}
