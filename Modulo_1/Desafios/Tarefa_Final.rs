/* Você foi desafiado a implementar uma função que realiza a compressão de uma string. 
A compressão consiste em contar a frequência de caracteres consecutivos 
e substituir essas sequências pelo caractere seguido do número de repetições. */


fn compress_string(str1: &str) -> String{
    let caracteres: Vec<char> = str1.chars().collect();

    if caracteres.is_empty() {
        return String::new();
    }

    let mut collect: char = caracteres[0];
    let mut contador:i32 = 1;
    let mut novo_array: Vec<char> = Vec::new();

    for c in 1..caracteres.len(){
        if collect == caracteres[c]{
            contador += 1;
        } 
        else{
            novo_array.push(collect);
            let char_contador = std::char::from_digit(contador as u32,10).unwrap();
            novo_array.push(char_contador);
            
            collect = caracteres[c];
            contador = 1;
        }
    }
    novo_array.push(collect);
    let char_contador = std::char::from_digit(contador as u32,10).unwrap();
    novo_array.push(char_contador);
    
    let string_comprimida: String = novo_array.iter().collect();

    if string_comprimida.len() > str1.len(){
        return str1.to_string();
    } 

    string_comprimida
}


fn main(){
    let exemplo1 = "aabcccccaaa";
    let exemplo2 = "abcdefgh";
    let exemplo3 = "aabbcc";
    
    println!("{}", compress_string(exemplo1)); 
    println!("{}", compress_string(exemplo2)); 
    println!("{}", compress_string(exemplo3)); 
}
