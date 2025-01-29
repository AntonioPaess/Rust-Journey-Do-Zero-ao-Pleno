/* Tarefa 2 - Nesta questão, você será desafiado a escrever uma função em Rust
 que receba um vetor de números inteiros e retorne o maior número encontrado nele. 
Essa função deve ser capaz de percorrer o vetor e encontrar o maior valor dentro dele. */

fn maior(lista: &[i32]) -> i32{
    let mut maior:i32 = lista[0];

    if lista.is_empty(){
        return 1;
    }
    

    for &i in lista{
        if i > maior{
            maior = i;
        }
    }
    return maior;
}


fn main(){
    let lista = vec! {1, 2, -2000, 80, 40, 34};

    if maior(&lista) == 1{
        println!("O vetor está vazio");
    }
    else{
        println!("O maior número é: {}", maior(&lista));
    }
} 