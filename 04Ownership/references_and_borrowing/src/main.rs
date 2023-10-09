fn main() {
    // Borrowing
    let s1 = String::from("hello");
    let len = calculte_lenth(&s1);

    println!("The lenth of '{}' is {}.", s1, len);

    // Mutable References
    let mut s = String::from("hello");
    change(&mut s); // the variable should be mutatable for the &mut to be passed

    let r1 = &mut s; // Only one &mut can be borrowed at a time
    // We can't have both mutable and imutable reference in the same scope

    // Multiple references
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // variables r1 and r2 will not be used after this point

    let r3 = &mut s; // no problem
    println!("{}", r3);
}

fn calculte_lenth(s: &String) -> usize {
    // function does not get ownership over argument
    // {s} is a reference to s1
    s.len()
}

fn change(some_string: &mut String) { // mutatable reference
    some_string.push_str(", world");
}
