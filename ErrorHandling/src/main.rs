use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        },
    };
}

fn main2() {
    let greeting_file = File::open("hello.txt").unwrap();
    // ---> retorna ok se valido ou panic!

    let greeting_file = File::open("hello.txt")
        .expect("hello.txt should be included in this project");
    //--> retorna ok se valido ou panic! com mensagem customizada
}