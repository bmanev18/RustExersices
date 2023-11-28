fn main() {
    let s = String::new();
    let data = "initial contents";

    let s = data.to_string(); // {.to_string()} is available to any type implementing the Display trait
                              //the method also works with literals
    let s = "initial contents".to_string();

    // Using {String::from()}function
    let s = String::from("Initial content");

    // Appending to a String
    let mut s = String::from("foo");
    s.push_str("bar"); // takes a reference of the argument

    s.push('1'); // pushes only one char

    // Concatination
    let s1 = String::from("Hello, ");
    let s2 = String::from("wolrd!");
    let s3 = s1 + &s2; // {s1} has been moved here
                       // {+} uses the {fn add(self, s: &str)}
                       // Takes ownership of {s1} and a copy of {s2}

    // Concatinating using format! micro
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{s1}-{s2}-{s3}");

    // Indexing into String
    for c in "Hello".chars(){
        println!("{c}");
    }
}
