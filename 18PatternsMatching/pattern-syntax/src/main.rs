fn main() {
    // Matching literals
    let x = 1;
    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    // Matching Named Variables
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {y}"),
        _ => println!("Defualt case, x = {:?}", x),
        // {y} from {Some(y)} is only local to the match scope and shadows the outer {let y = 10}
    }

    println!("At the end: x = {:?}, y = {:?}", x, y);
    //{y} in this case is the first {let y = 10}

    // Multiple Patterns
    let x = 1;

    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }
}
