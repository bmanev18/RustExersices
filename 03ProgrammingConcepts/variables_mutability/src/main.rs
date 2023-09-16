fn main() {
    
    // Shadowing
    let x = 5;
    let x = x + 1;

    {
        let x = x+2;
    
        println!("The value of X in the inner scope is: {x}");
    }
    
    println!("The value of X is: {x}");

    // Same name, different type
    let spaces = "    ";
    let spaces = spaces.len();
}