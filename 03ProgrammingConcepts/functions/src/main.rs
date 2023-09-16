fn main() {
    println!("Hello, world!");

    // Functions
    another_function();
    // Functions with parameters
    function_parameters(32);
    labeled_measurements(5, 'h');

    // Statements vs Expressions
    let x = 5; // Statement, returns void
               // let x = (let y = 6); // Error because statements does not return anything
    let y = {
        let x = 3;
        x + 1 // Expression returns value. ; is not needed when returning the value from the expression
    };

    let x = five();
    println!("The result of five() is: {x}");

    let x = plus_one(5);
    println!("The value of x is: {x}");
}

fn another_function() {
    println!("Another function")
}

fn function_parameters(x: i32) {
    println!("The value of X is: {x}")
}

fn labeled_measurements(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
