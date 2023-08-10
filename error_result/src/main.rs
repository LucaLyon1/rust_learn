use std::fs::File;
use std::io::ErrorKind;

fn main() {
    println!("Hello, Result!");

    //We try to read a file
    let greeting_file_result = File::open("hello.txt");

    let file_content = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => panic!("Error opening the file: {:?}", other_error),
        },
    };
}
