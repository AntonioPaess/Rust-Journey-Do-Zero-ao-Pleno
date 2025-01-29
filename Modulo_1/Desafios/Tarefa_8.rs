/* Implementar uma função em Rust que determine se uma string é uma permutação de outra. */

/*  eu poderia usar o as u8 pra fazer o casting mas... 
    ✅ Rust já entende que char pode ser comparado diretamente.
    ✅ A comparação usa os valores Unicode.
    ✅ Você não precisa converter char para u8 para fazer a ordenação.*/

    fn my_sort(vetor: &str) -> Vec<char>{
    let mut caracteres: Vec<char> = vetor.chars().collect();
    for i in 0..caracteres.len(){
           for j in i+1..caracteres.len(){  
            if caracteres[j] < caracteres[i]{

                let temp = caracteres[i];
                caracteres[i] = caracteres[j];
                caracteres[j] = temp;

            }

           }
    }
    return caracteres;
}

fn permutacao(str1: &str, str2: &str) -> bool{
    let string1: Vec<char> = my_sort(str1);
    let string2: Vec<char> = my_sort(str2); 

    if string1.len() != string2.len(){
        return false;
    }
    
    if string1 != string2{
        return false;
    }
    true
    
}



fn main(){
    let str1 = "abc";
    let str2 = "bca";
    
    if permutacao(str1, str2) {
        println!("As strings são permutações uma da outra.");
    } else {
        println!("As strings não são permutações uma da outra.");
    }

}