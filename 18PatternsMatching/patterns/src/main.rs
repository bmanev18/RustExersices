fn main() {
    // {match} Arms
    {
        let x = Some(5);
        match x {
            None => None,
            Some(i) => Some(i + 1),
        };
    }

    // Conditional {if let} expressions
    {
        let favorite_color: Option<&str> = None;
        let is_tuesday = false;
        let age: Result<u8, _> = "34".parse();

        if let Some(color) = favorite_color {
            println!("Using your favorite color, {color}, as the background");
        } else if is_tuesday {
            println!("Tuesday is a green day!");
        } else if let Ok(age) = age {
            if age > 30 {
                println!("Using purple as the backgroun color");
            } else {
                println!("Using orange as the background color");
            }
        } else {
            println!("Using blue as the background color");
        }
    }
}
