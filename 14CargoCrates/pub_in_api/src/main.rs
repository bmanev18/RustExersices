// Usual imports
/* use pub_in_api::kinds::PrimaryColor;
use pub_in_api::utils::mix; */


// using re-exports
use pub_in_api::PrimaryColor;
use pub_in_api::mix;


fn main() {
    let red = PrimaryColor::Red;
    let yellow = PrimaryColor::Yellow;

    mix(red, yellow);
}