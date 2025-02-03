/* 4 - Questão Fácil: Trocando Valores com Tuplas */

fn main() {
    let mut a = 10;
    let mut b = 20;

    // Use uma tupla para trocar os valores de `a` e `b`
    
    println!("Antes da troca: a = {}, b = {}", a, b);
    
    (a, b) = (b, a);

    println!("Depois da troca: a = {}, b = {}", a, b);
}
