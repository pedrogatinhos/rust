fn main() {
    // soma_numeros();
    start_e_soma();
}

fn start_e_soma(){
    let numero1 = 
     match recebe_numero() {
        Ok(result) => result,
        Err(_) => { println!("entrada invalida"); 
                    return start_e_soma();
        },
    };  
    let numero2 = 
     match recebe_numero() {
        Ok(result) => result,
        Err(_) => { println!("entrada invalida"); 
           return start_e_soma();
        },
}; 
    let result: u32 = numero1 + numero2;
    println!("{result}")
}

   
fn recebe_numero() -> Result<u32, ()> {
    let num_placeholder = rprompt::prompt_reply("digite um numero").unwrap();
    let num_placeholder = String::from(num_placeholder.trim());
    let num: u32 = match num_placeholder.parse::<u32>(){
        Ok(a) => a,
        Err(_) => return Err(()),
    };
    println!("executed");
    Ok(num)
}

