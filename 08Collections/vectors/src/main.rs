fn main() {
    // Store value of the same type next to each other in memory
    
    // if data is not known at creation
    let v:Vec<i32> = Vec::new(); 

    // if data is known at creation
    let mut v = vec![1, 2, 3];

    // Modifying. The vector should be mutable
    v.push(4);
    v.push(5);
    v.push(6);

    // Reding from vector returns a reference 
    // - using indexing. Not suitable for accessing invalid indexes
    let third:&i32 = &v[2]; // avoid taking ownership

    // - using {.get()}
    let third:Option<&i32> = v.get(2);
    match third {
        Some(i) => {println!("The third element is {i}")},
        None => {println!("There is no third element.")},
    }

    /* Cannot access a value at an index and then modify the vector! 
    The reference to the element will be pointing to deallocated memory */

    // Iterating over Vector
    let mut v = vec![100, 32, 57];
    for i in &v {
        println!("{i}");
    }

    for i in &mut v {
        *i +=50;
    }
}
