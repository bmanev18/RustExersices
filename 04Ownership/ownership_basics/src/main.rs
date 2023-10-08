fn main() {
    // Variable scope
    let s = "hello"; // s is valid from this point forward
    println!("{}", s);

    // The String type
    let mut s = String::from("hello");
    s.push_str(", world"); // push_str appends a literal to a String
    println!("{}", s);

    // Variables and Data Interacting with Move
    let x = 5; // x = 5
    let y = x; // y = 5 (when basic type -> makes copy)

    let s1 = String::from("hello"); // String contains pointer, length and capacity
    let s2 = s1; // Stack information is copied (len, cap). Both pointers are the same

    // println!("{}, world", s1); // After s2 = s1 -> s1 is not valid any more (s1 was moved to s2)
    // A value can have only one owner!

    // Variables and Data Interacting with Clone
    let s1 = String::from("hello");
    let s2 = s1.clone(); // Heap data is copied. s2 now has a different pointer for the same data
    println!("s1 = {}, s2 = {}", s1, s2);

    // Ownership and Functions
    let s = String::from("hello");
    takes_ownership(s); /* The function takes an ownership of the value
                        thus {s} is no longer valid */

    let x = 5;
    make_copy(x); /* {x} will move into the function, but :i32 has a Copy trait,
                  so {x} is still valid after this line */

    // Return Values ans Scope
    let s1 = gives_ownership();
    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2); // s2 is moved into the method

    // only s1 and s3 are valid to this point
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn make_copy(some_int: i32) {
    println!("{}", some_int);
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn gives_ownership() -> String {
    let some_string = String::from("yours");
    some_string // no return needed
}
