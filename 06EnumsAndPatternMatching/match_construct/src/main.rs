#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    // using enum inside of enum
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    // match control flow construct
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            // state is a variable which can be used in the body of the match case, derived from Coin::Quarter declaration
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn main() {
    value_in_cents(Coin::Quarter(UsState::Alaska));

    // Match construction with Option<T>
    fn plus_one(x: Option<i32>) -> Option<i32> {
        // {match} should cover all cases
        match x {
            None => None,
            Some(i) => Some(i + 1), // {i} is the value which will be inside if {x} matches Option::Some
                                    // Some(5) => Some(5 + 1)
        }
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    // Match construction with catch-all pattern
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_had(),
        7 => remove_fancy_hat(),
        other => move_player(other), // {other} will be equal to the value that doesn't match. Can be used in the case body
    }

    // Match construction with {_} placeholder
    match dice_roll {
        3 => add_fancy_had(),
        7 => remove_fancy_hat(),
        _ => reroll(), // {_} placeholder catches all other matches, but can't be used later like {other}
        // _ => (), // It can also do nothing
    }

    fn add_fancy_had() {}
    fn remove_fancy_hat() {}
    fn move_player(other: u8) {}
    fn reroll() {}

}
