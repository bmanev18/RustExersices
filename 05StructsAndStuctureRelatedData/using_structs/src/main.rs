#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let width = 30;
    let heigth = 50;

    println!(
        "The area of the rectangle is {} square pixels.(Using basic signature)",
        area_old(width, heigth)
    );

    // Calculating area using a tuple for the measurements
    let rect1 = (30, 50);
    println!(
        "The area of the rectangle is {} square pixels.(Using tuples)",
        area_tuple(rect1)
    );

    // Calculating the area using a struct for the measurements
    let rectangle = Rectangle {
        width: 30,
        height: 50,
    };
    println!(
        "The area of the rectangle is {} square pixels(Using struct)",
        area_struct(&rectangle)
    );

    // Printing an instance of a struct
    println!("rectangle is {:?}", rectangle);

    // Printing an instance of a struct with the debug macro which returns an ownership
    dbg!(&rectangle);
}

fn area_old(width: u32, height: u32) -> u32 {
    width * height
}

fn area_tuple(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area_struct(rectangle: &Rectangle) -> u32 {
    rectangle.height * rectangle.width
}
