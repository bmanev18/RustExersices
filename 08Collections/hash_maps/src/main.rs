use std::collections::HashMap;

fn main() {
    // Creating a new hash map
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    // get() returns a refference to the value, so copied() does a deep copy

    // Itterate over
    for (key, value) in &scores {
        println!("{key}: {value}"); // printing in arbitrary order
    }

    // Updating values
    // 1. Overwritting a value
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);
    println!("{:?}", scores);

    // 2. Add value only if a key isn't present
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50); // key exists      -> do not insert
    scores.entry(String::from("Blue")).or_insert(50); // key doesn't exist -> do not insert
    println!("{:?}", scores);

    // 3. Update value based on previous value
    let text = "hello world wonderful world";
    
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0); 
        // or_insert() returns a mutable reference to the value
        // count: &mut i32 -> mut reference
        *count+=1;
        // have to dereference and then increment the value
    }
    println!("{:?}", map);

}
