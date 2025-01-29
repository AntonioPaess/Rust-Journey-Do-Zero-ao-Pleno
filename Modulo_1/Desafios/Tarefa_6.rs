/* Escreva uma função em Rust que recebe um vetor de notas como parâmetro e calcula a média dessas notas.
 A função deve retornar a média aritmética das notas */

fn media(notas: Vec<f64>) -> f64{
    let total:f64 = notas.len() as f64;
    let mut soma:f64 = 0.0;
    for nota in notas{
        soma += nota;
    }
    let media:f64 = soma/total;
    media 
}


 fn main(){
    let notas: Vec<f64> = vec![6.0, 8.0, 9.0, 2.0];
    println!("A media das notas é: {}", media(notas));
}