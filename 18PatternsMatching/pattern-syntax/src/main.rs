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

    // 2. Destructuring Enums
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    let msg = Message::ChangeColor(0, 160, 255);

    match msg {
        Message::Quit => {
            println!("The Quit variant has not data to destructure");
        }
        Message::Move { x, y } => {
            println!("Move in the x direction {x} and in the y direction {y}");
        }
        Message::Write(text) => {
            println!("Text message: {text}");
        }
        Message::ChangeColor(r, g, b) => {
            println!("Change the color ro red {r}, green {g}, and blue {b}");
        }
    }

    // 3. Destructuring Nested Enums
    {
        enum Color {
            Rgb(i32, i32, i32),
            Hsv(i32, i32, i32),
        }

        enum Message {
            Quit,
            Move { x: i32, y: i32 },
            Write(String),
            ChangeColor(Color),
        }

        let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));

        match msg {
            Message::ChangeColor(Color::Rgb(r, g, b)) => {
                println!("Change the color ro red {r}, green {g}, and blue {b}");
            }
            Message::ChangeColor(Color::Hsv(h, s, v)) => {
                println!("Change the color to hue {h}, saturation {s}, value {v}");
            }
            _ => (),
        }
    }

    // 4. Destructuring Structs and Tuples
    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });

    // Ignoring values in Patterns
    fn foo(_: i32, y: i32) {
        println!("This code only uses the y parameter: {}", y);
    }

    foo(3, 4);

    // Ignoring Parts of a Value with a Nested {_}
    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }

    println!("setting is {:?}", setting_value);

    //multiple {_}
    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, _, third, _, fifth) => {
            println!("Some numbers: {first}, {third}, {fifth}")
        }
    }

    // Ignoring Remaining Prts of a value with {..}
    struct D3Point {
        x: i32,
        y: i32,
        z: i32,
    }

    let origin = D3Point { x: 0, y: 0, z: 0 };
    match origin {
        D3Point { x, .. } => {
            println!("x is {x}");
        } // {D3Point {x, ..}} is the same as {D3Point {x, y: _, z: _}}
    }

    let numbers = (2, 4, 8, 16, 32);
    match numbers {
        (first, .., last) => {
            println!("Some numbers: {first}, {last}");
        } // {..} can be used once and the usage should be clear
    }

    // Extra Conditions with Match Guards
    let num = Some(4);

    match num {
        Some(x) if x % 2 == 0 => println!("The number {} is even", x),
        Some(x) => println!("The number {} is odd", x),
        None => (),
    }

    //using an external to the {match} variable, inside
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(n) if n == y => println!("Matched, n = {n}"),
        _ => println!("Default case, x = {:?}", x),
    }

    //{|} with match guard
    let x = 4;
    let y = false;

    match x {
        4 | 5 | 6 if y => println!("yes"),
        _ => println!("no"),
    }
}
