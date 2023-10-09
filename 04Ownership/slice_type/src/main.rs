fn main() {
    // Without the Slice type
    let mut s = String::from("hello world"); // because of .clear() the variable shiuld be mut

    let word = first_word(&s); // word will get the value 5

    s.clear(); // this empties the String, making it equal to ""
               // word still has the value 5 here, but there's no more string that
               // we could meaningfully use the value 5 with. word is now totally invalid!

    // The Slice type
    let s = String::from("hello world");

    let hello = &s[0..5]; // can be writen as [..5]
    let world = &s[6..11]; // can be writen as [6..]
    let full_string = &s[..]; // a slice of the whole string

    // Improving first_word function
    let mut s = String::from("hello, world");

    let word = first_word_improved(&s);
    // {word} points at some place in {s}
    //s.clear(); The println! after the call to clear uses the reference in word, so the immutable reference must still be active.
    println!("the first word is: {}", word);

    // Other slices
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    assert_eq!(slice, &[2, 3]);
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        // .enumerate() returns a tuple of index and element reference
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn first_word_improved(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
