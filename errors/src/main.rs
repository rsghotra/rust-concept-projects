use std::fs::File;
use std::error::Error;
use std::io::{self, ErrorKind, Read};

fn main() -> Result<(), Box<dyn Error>> {

    //way 1 - using nested match
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fs) => fs,
                Err(e) => panic!("Problem CREATING the file {e:?}"),
            },
            _ => {
                panic!("Problem OPENING the file: {error:?}");
            }
        },
    };

    //way 2 - using closures
    let greeting_file_result2 = File::open("hello2.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello2.txt").unwrap_or_else(|error| {
                panic!("Problem CREATING the file {error:?}");
            })
        } else {
            panic!("Problem OPENING the file {error:?}");
        }
    });

    //way3 - shortcut to panic - unwrap
    let greeting_file_result3 = File::open("hello3.txt").unwrap();

    //way4 - shortcut to panic - expect

    let greeting_file_result4 = File::open("hello4.txt").expect("hello4.txt should be part of this project");

    let greeting_file = File::open("hello5.txt");

    Ok(())
}


fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("username.txt");

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

//usinger operator ?

fn read_username_from_file2() -> Result<String, io::Error> {
    let mut username_file = File::open("hello3.txt")?;

    let mut username = String::new();

    username_file.read_to_string(&mut username)?;

    Ok(username)

}

fn read_username_from_file3() -> Result<String, io::Error> {
    
    let mut username = String::new();

    File::open("hello3.txt")?.read_to_string(&mut username)?;

    Ok(username)

}