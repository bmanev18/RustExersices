fn main() {
    // if/else
    let number = 3;
    if number < 5 {
        println!("true");
    } else {
        println!("false");
    }

    // if/if else/else
    if number != 0 {
        println!("Different than 0");
    } else if number < 0 {
        println!("Number less than 0");
    } else {
        println!("Number is more than 0");
    }

    // if in let Statements
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of the number is: {number}");

    // loops
    // 1. Infinite loop
    loop {
        println!("again");
        break; // keyword to break the loop
    }

    // 2. Returning values from loop
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    println!("the result is: {result}\n");

    // 3. Labeling loops
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}\n");

    // 4. While loop
    let mut count = 3;
    while count != 0 {
        println!("{count}");
        count -= 1;
    }
    println!();

    // 5.1 For loop through collection
    let arr = [1, 2, 3, 4, 5];
    let mut index = 0;

    while index < 5 {
        println!("The value is: {}", arr[index]);
        index += 1;
    }
    println!();

    // 5.2 "Enchanced" For loop
    let arr = [1, 2, 3, 4, 5];

    for element in arr {
        println!("the value is: {element}")
    }
    println!();

    // 5.3 For loop with Range(x..y) => x >= count < y !
    for number in (1..4).rev() {
        println!("{number}");
    }
    println!();
}
