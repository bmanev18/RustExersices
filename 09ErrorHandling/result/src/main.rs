#![allow(unused)]

use std::{
    fs::{self, File},
    io::{self, Error, ErrorKind, Read},
};
fn main() {
    // Recoverable errors with Result
    let greeting_file_result = File::open("hello.txt");
    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file {:?}", error),
    };

    // Matching on different Errors
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error)
            }
        },
    };

    // Alternative way
    let greeating_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|e| {
                panic!("Problem creating the file: {:?}", e);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });

    // Using expect instead
    let greeting_file =
        File::open("hello.txt").expect("hello.txt should be included in this project");

    // Propagating errors
    read_username_from_file();
    read_username_from_file_short();
    read_username_from_file_shorter();
    read_username_from_file_shortest();
}

fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

fn read_username_from_file_short() -> Result<String, Error> {
    let mut username_file = File::open("hello.txt")?; 
    // ? if Err will break and propagate
    // Can only be used in functions with return type that implements FromResidual
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

fn read_username_from_file_shorter() -> Result<String, Error> {
    let mut username = String::new();

    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username)
}

fn read_username_from_file_shortest() -> Result<String, Error> {
    fs::read_to_string("hello.txt")
    // std provides a function that opens the file, creates a new String, reads the contents of the file, puts the contents into that String, and returns it.
}
