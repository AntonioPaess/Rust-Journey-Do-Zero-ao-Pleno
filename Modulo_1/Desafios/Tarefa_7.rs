/* Implemente um algoritmo para determinar se uma string possui todos os caracteres únicos.
 E se você não puder usar estruturas de dados? */

 fn tem_caracteres_unicos(entrada: &str) -> bool{
    let caracteres: Vec<char> = entrada.chars().collect();

    for i in 0..caracteres.len(){
        for j in (i + 1)..caracteres.len(){
            if caracteres[i] == caracteres[j]{
                return false;
            }
        }
    }

    true
}

 fn main(){
    let palavra = "cateto";

    if tem_caracteres_unicos(teste) == true{
        println!("A string possui todos os caracteres únicos");
    }
    else{
        println!("A string não possui todos os caracteres únicos.");
    }
 }