use std::{ops::Add, process::Output};

// 1.Associated Types vs Generic Types for Trait
struct Counter;

// 1.1 Using generic types {<T>}
//-> Allows for multiple trait implementation on the same struct with a diferent <T>
trait Iterate<T> {
    fn next(&mut self) -> Option<T>;
}
// -> First implementation of Itarate<T> for Counter
impl Iterate<i32> for Counter {
    fn next(&mut self) -> Option<i32> {
        unimplemented!();
    }
}

// -> Second implementation of Itarate<T> for Counter
impl Iterate<u8> for Counter {
    fn next(&mut self) -> Option<u8> {
        unimplemented!();
    }
}

// 1.2 Using Associative Type
// -> Limits the trait implementation to only one per struct
impl Iterator for Counter {
    type Item = u32;
    // a specific type inside the trait
    fn next(&mut self) -> Option<Self::Item> {
        unimplemented!();
    }
}

// 2. Default Generic Type Parameters and Operator Overloading

// 2.1 Default Generic Type of type {Self}
// declaration of std::ops::Add
// Rhs will by default be the same type as the caller
/* trait Add<Rhs=Self> {
    type Output;

    fn add(self, rhs: Rhs) -> Self::Output;
} */
#[derive(PartialEq, Debug)]
pub struct Point {
    pub x: i32,
    pub y: i32,
    // public just for the example in main.rs
}

impl Add for Point {
    // Rhs is type Point
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

// Check main.rs

// 2.2 Default Generic Type of different than {Self} type
struct Milimiters(u32);
struct Meters(u32);

impl Add<Meters> for Milimiters {
    // {<Meters>} overrides the default type or {Rhs}
    type Output = Milimiters;

       fn add(self, rhs: Meters) -> Self::Output {
           Milimiters(self.0 + (rhs.0 * 1000))
           // Since the structs have touples, {.0} is required for accessing it
       } 
}

