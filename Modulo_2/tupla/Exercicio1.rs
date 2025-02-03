/* 1 - Questao Fácil: Acessando elementos de uma tupla */


fn main() {
    let pessoa = ("Alice", 25, 1.65);
    
    // Complete o código abaixo para imprimir: 
    // "Nome: Alice, Idade: 25, Altura: 1.65"
    println!("Nome: {}, Idade: {}, Altura: {}", pessoa.0, pessoa.1, pessoa.2);
}
