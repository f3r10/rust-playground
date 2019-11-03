use std::fs;
use std::fs::File;
use std::io;
use std::io::Read;
pub fn error_handling() {
    let f = File::open("hello.txt");
    match f {
        Ok(file) =>
            println!("success result"),
        Err(error) =>
            println!("problem opening the file: {:?}", error),
    };
    let name = read_name_from_file("hello.txt");
    match name {
        Ok(name) => println!("the name is: {:?}", name),
        Err(error) => println!("unable to read the name from: {:?}", "hello.txt"),
    };
    // let f2 = File::open("hello.txt");
    // f2.unwrap();
    // let f3 = File::open("hello.txt");
    // f3.expect("error to open file");
}

fn read_name_from_file(filename: &str) -> Result<String, io::Error> {
    // opcion 1
    // let mut f = File::open(filename)?;
    // let mut s = String::new();
    // f.read_to_string(&mut s)?;
    // Ok(s)
    // Opcion 2
    // let mut s = String::new();
    // File::open(filename)?.read_to_string(&mut s)?;
    // Ok(s)
    fs::read_to_string(filename)
}
