fn main(){
    let tupla = (12, "valores", 3.14, (1,2,3));
    
    // cada letra vai ficar com um valor especifico da tupla.
    let (a, b, c, d) = tupla;

    // acessando os valores.
    println!("{}", tupla.1);

    // Acessando o elemento 2 da tupla que está dentro da tupla.
    println!("{}", (tupla.3).1);

    println!("O valor de a é: {}", a);
    println!("O valor de b é: {}", b);
    println!("O valor de c é: {}", c);

    // para printar uma tupla tem que utilizar a sintaxe {:?} ou importar o use std::fmt::Display::fmt
    // pode ser usado a sintaxe: {:#?} para imprimiar na vertical
    println!("O valor de d é: {:#?}", d);

}