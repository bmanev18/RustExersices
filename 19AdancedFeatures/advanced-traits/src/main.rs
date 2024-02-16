use advanced_traits::Point;

fn main() {
    // Overriding operator Add for Point
    assert_eq!(Point{x:1, y:0} + Point{x:2, y:3}, Point{x:3, y:3});
}