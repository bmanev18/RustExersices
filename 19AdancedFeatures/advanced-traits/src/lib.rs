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
