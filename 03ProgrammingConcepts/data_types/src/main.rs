fn main() {
    // Define explicitly the type if many are posibble
    let guess: u32 = "42".parse().expect("Not a number");

    // Scalar types
    // 1. Integers
    let int = 5;
    let dec = 98_222;
    let hex = 0xff;
    let oct = 0o77;
    let bin = 0b1111_0000;
    let byte = b'A';

    // 2. Floating-Point types
    let float = 3.14;
    let float: f32 = 2.0;

    // 3. Boolean and char
    let bool = true;
    let char = 'c'; //4 bites

    // Compound types
    // 1. Tuple - different types, fixed size
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup; // x, y, x will be seperate variables. Variable destructoring
    let second = tup.1; // i = 0, 1, 2
    let empty: () = (); // an empty tuple is called "unit"

    // 2. Array - same type, fixed size
    let arr = [1, 2, 3, 4, 5]; // implicit
    let arr: [i32; 5] = [1, 2, 3, 4, 5]; // explicit

    let a = [3; 5]; // [value, count] = [3, 3, 3, 3, 3]

    let first = arr[0]; // first = 1;
    
}
