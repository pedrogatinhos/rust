use std::collections::HashMap;

fn main() {
    let mut nums: HashMap<i32, i32> = HashMap::new();
    let mut vetor = vec![1, 2, 3, 4, 5, 4, 4, 9, 2, 5,9, 9,9 ,9 ,9 ];
    let mut biggest_repeat: i32 = 0;

    for item in vetor.iter() {
        match nums.get(item) {
        Some(T) => {
            let ac = *nums.get(item).unwrap();
            nums.insert(*item, ac+1);   //vetor.iter returnn refs, must be actual values, that's why *
            biggest_repeat = if *nums.get(item).unwrap() > biggest_repeat {*item}  else { biggest_repeat };
        },
        _=> {
            println!("registering new key");
            nums.insert(*item, 1);  
        },
      }
    }
    println!("the most repeated item is: {biggest_repeat}");
}