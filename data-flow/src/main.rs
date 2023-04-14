use std::io;
fn main() {
    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is: {y}");
    let fnvalue = num();
    println!("The value of y is: {fnvalue}");
    flow();
    countloop();
    forarray();
    rand();

}

fn num() -> u32{
    // 9 -- works
    return 9;

}

fn flow() {
    let number = 3;
    if number < 5 && 2 > 1 {
        println!("condition was true opts");
    } else {
        println!("condition was false");
    }
}
fn countloop(){
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    println!("value of result if: {result}")
}
fn forarray(){

    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }
}
fn rand(){
    for number in (1..4).rev(){
        println!("counting!!")

    }
}
