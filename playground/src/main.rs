use std::simd::u8x32;

use rprompt;

fn main() {
   
  soma_numeros();

}

fn soma_numeros() {
    let mut num_placeholder1 = rprompt::prompt_reply("Digite um numero ");
    let _num1: u32 = match num_placeholder.trim(){
        Ok(`Ok`) => println!("ok"),
        Err(_) =>println!("erro"),
    }; 
    let mut _num_placeholder2: String = rprompt::prompt_reply("Digite outro numero ").unwrap();

    
}