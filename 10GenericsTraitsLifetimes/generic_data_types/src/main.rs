fn largest<T>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        /* if item > largest {
            largest = item;
        } */
    }

    largest
}

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    // {impl <T>} tells the compiler that <T> is a generic type.
    fn x(&self) -> &T {
        &self.x
    }
}

struct BetterPoint<T, U> {
    x: T,
    y: U,
}

impl<T, U> BetterPoint<T, U> {
    fn mixup<A, B>(self, other_point: BetterPoint<A, B>) -> BetterPoint<T, B> {
        // New generic types can be introduced after method name and will be relevant only for this method body
        BetterPoint {
            x: self.x,
            y: other_point.y,
        }
    }
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);
    println!("The largest char is {}", result);

    // In Struct Definition
    let integer_point = Point { x: 5, y: 10 };
    let float_point = Point { x: 1.0, y: 4.0 };

    // since {x} and {y} have the same generic <T>, then they have to be the same type
    //let wont_work__point = Point{x:5, y:4.0};

    // the solution is to define two generic types if two imput types are expected
    let integer_and_float = BetterPoint { x: 5, y: 4.0 };

    let p1 = BetterPoint{x:5, y:10.4};
    let p2 = BetterPoint{x:"Hello", y:'c'};
    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x,p3.y);
}
