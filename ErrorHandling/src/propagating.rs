use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

fn read_username_from_file2() -> Result<String, io::Error> {
    let mut username = String::new();

    File::open("hello.txt")?.read_to_string(&mut username)?;

    Ok(username)
}

/*The ? placed after a Result value is defined to work in almost the same way as the match expressions we defined 
to handle the Result values in Listing 9-6. If the value of the Result is an Ok, the value inside the Ok will get 
returned from this expression, and the program will continue. If the value is an Err, the Err will be
returned from the whole function as if we had used the return keyword so the error value gets propagated to the 
calling code. */