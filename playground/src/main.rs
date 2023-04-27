fn main() {
    soma_numeros();
}

fn soma_numeros() {
    loop {
        match get_nums() {
            Ok(result) => {
                println!("O resultado é {}", result);
                break;
            },
            Err(_) => println!("Erro ao obter os números"),
        }
    }
}

fn get_nums() -> Result<u32, ()> {
    let num1_placeholder = rprompt::prompt_reply("Digite um numero ").unwrap();
    let num1 = String::from(num1_placeholder.trim());
    let num1 = match num1.parse::<u32>() {
        Ok(a) => a,
        Err(_) => return Err(()),
    };

    println!("parsed");
    println!("{num1}");

    let num2_placeholder = rprompt::prompt_reply("Digite um numero ").unwrap();
    let num2 = String::from(num2_placeholder.trim());
    let num2: u32 = match num2.parse::<u32>() {
        Ok(a) => a,
        Err(_) => return Err(()),
    };
    let result = num1 + num2;
    Ok(result)
}