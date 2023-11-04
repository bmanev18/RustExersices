fn main() {
    let config_max = Some(3u8);

    //  if true for a variable (with match)
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    }

    //  if true with {if let}
    if let Some(max) = config_max{
        println!("The maximum is configured to be {}", max);
    } else {
        println!("There is no maximum");
        
    }
}
