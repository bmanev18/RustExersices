fn main() {
    // calling panic!
    panic!("crash and burn");

    // panic backtrace
    let v = vec![1, 2, 3];
    //v[99]; //-> panicked at 'index out of bounds: the len is 3 but the index is 99'
}
