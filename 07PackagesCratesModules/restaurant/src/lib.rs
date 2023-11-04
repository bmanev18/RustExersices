mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitinglist() {}
        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}
        fn serve_order() {}
        fn take_payment() {}
    }
}
/*
crate
|__ front_of_house
|   |__ hosting
|   |   |__ add_to_waitinglist()
|   |   |__ seat_at_table()
|   |__ serving
|       |__ take_order()
|       |__ serve_order()
|       |__ take_payment()
|__ eat_at_restaurant()
|__ delicer_order()
|__ back_of_house
|   |__ fix_inccorect_order()
|   |__ cook_order()
|__ order()

    */

// Absolute, Relative paths and Use:
pub use crate::front_of_house::hosting;
// {use} should be in the same module scope as its use
// {pub use} allows hosting to be used directly in a parent without writing the whole path
pub fn eat_at_restaurant() {
    // Absulute path
    crate::front_of_house::hosting::add_to_waitinglist();

    // Relative path
    front_of_house::hosting::add_to_waitinglist();

    // Use
    hosting::add_to_waitinglist(); // a good practive is to bring the parent and not the child directly
}

// Starting relative Paths with super
fn deliver_order() {}

mod back_of_house {
    fn fix_inccorect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}

    pub enum Appetizer {
        Soup,
        Salad,
    } // All enum variants are public by default
}

pub fn order() {
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}
