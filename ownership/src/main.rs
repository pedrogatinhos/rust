fn main() {
    // let mut heapstring = String::from("hello");
    // heapstring.push_str(", world!");
    // println!("{heapstring}");

    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
    let s_adress = &s1 as *const String;
    println!("adress is {:p}", s_adress)
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
