struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user_old(email: String, username: String) -> User {
    User {
        active: true,
        username: username,
        email: email,
        sign_in_count: 1,
    }
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username, // if arguments have the same names as the fields
        email,
        sign_in_count: 1,
    }
}

struct Color(i32, i32, i32);

fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com"); // only if user1 is mut

    let user2 = User {
        active: user1.active,
        username: user1.username, // user1.username is borrowed
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    };
    // how to acheive the same result with less code

    let user3 = User {
        email: String::from("another@example.com"),
        ..user2 // cannot be user1 because user2.username has ownership of user1.username
    };

    // Using Tuple Structs without Fields to Create Different Types
    let black = Color(0, 0, 0);

    // Unit-Like Structs Without Any Fields
    struct AlwaysEqual;
    let object = AlwaysEqual;

    


}
