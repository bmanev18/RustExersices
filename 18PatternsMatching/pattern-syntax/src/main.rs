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

    // Matching a range of values
    let x = 5;

    match x {
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }
    //usin {..} instead of multiple {|}
    //only allowed with {char} and numeric values

    // Destructuring to Break Apart Values
    // 1. Destructuring Structs
    struct Point {
        x: i32,
        y: i32,
    }

    let p = Point { x: 0, y: 7 };

    let Point { x: a, y: b } = p;
    assert_eq!(0, a);
    assert_eq!(7, b);
    //{a = x} and {b = y}

    //Since it's comanly used =>
    let Point { x, y } = p;
    assert_eq!(0, x);
    assert_eq!(7, y);

    //Matching
    match p {
        Point { x, y: 0 } => println!("On the x axis at {x}"),
        Point { x: 0, y } => println!("On the y axis at {y}"),
        Point { x, y } => println!("On neither axis"),
    }
}
