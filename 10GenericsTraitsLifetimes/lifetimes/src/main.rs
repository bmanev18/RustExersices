fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("the longest string is {}", result);

    // Struct lifetime
    let novel = String::from("call me Ishmael. Some years ago...");
    let first_sentance = novel.split('.').next().expect("Coud not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentance,
    };
}

// the returned reference will be valid as long as both the parameters are valid.
//This is the relationship between lifetimes of the parameters and the return value.
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// Struct lifetime
struct ImportantExcerpt<'a> {
    part: &'a str,
}
