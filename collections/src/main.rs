fn main() {
    println!("Hello, world!");
    let mut vetor: Vec<i32> = Vec::new();
    let vetor2 = vec![1,2,3,4];
    vetor.push(1);
    vetor.push(2);

    let referenceVetor: &i32 = &vetor[0];
    println!("{referenceVetor}");
    
    let referenceVetor2: Option<&i32> = vetor.get(1);
    match referenceVetor2{
        Some(value) => println!("{value}"),
        None => println!("Ooops"),
    };
}
