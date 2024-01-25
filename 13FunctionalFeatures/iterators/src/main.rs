fn main() {
    // Methods that Consume the Iterator
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();

    let total: i32 = v1_iter.sum();

    // Methods that Produce Other Iterator
    let v2_iter = vec![1, 2, 3].into_iter();
    let mapped_values: Vec<_> = v2_iter.map(|x| x + 1).collect();
    // when .collect is called, the values are mapped
}
