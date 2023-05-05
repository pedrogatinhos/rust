use std::collections::HashMap;

fn main() {;
    let mut nums: HashMap<i32, i32> = HashMap::new();
    let mut vetor = vec![1, 2, 3, 4, 5, 4, 4, 9, 2, 5];
    let mut biggest_repeat: i32 = vetor[0];

    for item in vetor.iter() {
        match nums.get(item) {
        Some(T) => {
            nums.insert(1,2);
            biggest_repeat = if *item > biggest_repeat {*item}  else { biggest_repeat };
            println!("found");
        },
        _=> {println!("not");
            nums.insert(*item, *item);  //vetor.iter() return refs, must use actual values
            println!("inserted");
        },
      }
    }
    println!("{biggest_repeat}");
}