/* 2️ - Questão Intermediária: Função que Retorna uma Tupla 
Enunciado:
Escreva uma função chamada calcular_area_perimetro que recebe a 
base e a altura de um retângulo como parâmetros (f32) e 
retorna uma tupla contendo a área e o perímetro do retângulo.

Depois, no main, utilize essa função para calcular e imprimir a 
área e o perímetro de um retângulo de base 5.0 e altura 3.0. */

fn calcular_area_perimetro(base: f32, altura:f32) -> (f32, f32){
    let area:f32 = base * altura;
    let perimetro:f32 = 2.0 * (base + altura);

    (area, perimetro)
}

fn main(){
    let base:f32 = 5.0;
    let altura:f32 = 3.0;
    println!("Área: {}, Perímetro: {}", calcular_area_perimetro(base, altura).0, calcular_area_perimetro(base, altura).1);
}