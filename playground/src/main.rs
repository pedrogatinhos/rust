use rprompt;


fn main() {
   
  soma_numeros();
  

}

fn soma_numeros() {
    fn test_fun() {
        println!("numero invalido, reiniciando operação");
        get_nums(test_fun);
      };
      get_nums(test_fun);
      
}
fn get_nums(test_fun: fn()) => u32 {
    let num1_placeholder = rprompt::prompt_reply("Digite um numero ").unwrap();
    let num1 = String::from(num1_placeholder.trim());
    let num1 = match num1.parse::<u32>() {
        Ok(a) => a,
        Err(_) =>  return test_fun(),
    };

    println!("parsed");
    println!("{num1}");

    let num2_placeholder = rprompt::prompt_reply("Digite um numero ").unwrap();
    let num2 = String::from(num2_placeholder.trim());
    let num2: u32 = match num2.parse::<u32>() {
        Ok(a) => a,
        Err(_) =>  return test_fun(),
    };
    result = num1 + num2;
    result

}