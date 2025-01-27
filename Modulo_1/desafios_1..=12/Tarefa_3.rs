/* Escreva uma função em Rust que verifica se um número é primo, retornando "true" ou "false". 
Verifique se o número é divisível por outros números usando loop e condições. */

fn eh_primo(numero: i32) -> bool{
    if numero <= 1{
        return false;
    }

    let limite = (numero as f32).sqrt().ceil() as i32;

    for i in 2..=limite{
        if numero % i == 0{
            return false;
        }
    }

    return true;
    
}



fn main(){
    let numero = 13;
    if eh_primo(numero) == true{
        
        println!("{} é primo", numero);
    }
    else{
        println!("{} não é primo", numero);      
    }
}