fn main(){
    let stringheap =  String::from("hello world");
    
   let result = first_word(&stringheap);
    println!("{result}")
}

fn first_word(string: &String) -> &str {
    let bytes = string.as_bytes();

    for (item, &char) in bytes.iter().enumerate(){
        if char == b' ' {
            return &string[0..item];
        }
    }
    &string[..]
}


/* extra code */
/*
fn main(){
    let string = "hello world";
    let len = string.len();
    println!("{string}"); // output: hello world
		println!("{len}")     //  output: 11
} 
        #DIFF

fn main(){
    let stringheap =  String::from("hello world");
    let capacity = stringheap.capacity();
    let len = stringheap.len();

    println!("{stringheap}");  //output: hello world
	  println!("{capacity}");     //output: 11
		println!("{len}");           //output: 11
}
 */