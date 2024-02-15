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

    // {while let} conditional loops
    {
        let mut stack = Vec::new();

        stack.push(1);
        stack.push(2);
        stack.push(3);

        //while the popped value matches as Some
        //while !stack.isEmpty()
        while let Some(top) = stack.pop() {
            println!("{}", top)
        }
    }

    // {for} loops
    {
        let v = vec!['a', 'b', 'c'];

        for (index, value) in v.iter().enumerate() {
            println!("{} is at index {}", index, value);
        }
    }

    // {let} Statement
    {
        // let PATTERN = EXPRESSION
        let (x, y, z) = (1, 2, 3);
        // the name is a pattern, which matches with the expression
    }
}
