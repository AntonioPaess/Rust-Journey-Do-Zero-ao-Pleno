/* 3️ - Questão Avançada: Desestruturação de Tuplas 
Enunciado:
No código abaixo, há uma função chamada analisar_numeros 
que recebe três números inteiros e retorna uma tupla com:

O maior número
O menor número
A soma de todos os números

Seu objetivo é completar a implementação da função e utilizá-la 
no main para analisar os números 4, 9, 2.*/

fn analisar_numeros(a:i32, b:i32, c:i32) -> (i32, i32, i32){
let soma:i32 = a + b + c;
let maior = if a >= b && a >= c{
    a
}else if b >= a && b >= c{
    b
}else{
    c
};

let menor = if a <= b && a <= c {
    a
} else if b <= a && b <= c {
    b
} else {
    c
};
    
    (maior, menor, soma)
}

fn main(){
    let numeros = analisar_numeros(4, 9, 2);
    println!("Maior: {}, Menor: {}, Soma: {}", numeros.0, numeros.1, numeros.2);
}

