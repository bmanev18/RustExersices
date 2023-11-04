use rand::Rng;
use std::collections::HashMap;
use std::io::Result as IoResult; // import as

// If using multiple modules in the same crate level: Nested path
use std::{cmp::Ordering, io};

// If using a module and one of their modules
use std::io::Write; // This brings both std::io and std::io::Write

// The global operator
use std::collections::*;
fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);

    // using external dependencies from Cargo.toml
    let secret_number = rand::thread_rng().gen_range(1..=100);
}

fn function() -> IoResult<()> {
    IoResult::Ok(())
}
