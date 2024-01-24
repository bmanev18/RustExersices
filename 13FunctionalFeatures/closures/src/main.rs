use std::thread;

fn main() {
    // Borrowing immutable closure
    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    let only_borrows = |x| println!("From closure: {:?} | x = {x}", list);

    println!("Before calling closure: {:?}", list);
    only_borrows(4);
    println!("After calling closure: {:?}", list);
    println!("--------------------------------------\n");

    // Borrowing mutable closure
    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    let mut borrows_mutability = || list.push(7);

    borrows_mutability();
    println!("Before defining closure: {:?}", list);
    println!("--------------------------------------\n");

    // Taking ownership closure
    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    thread::spawn(move || println!("From thread: {:?}", list))
        .join()
        .unwrap();
}
