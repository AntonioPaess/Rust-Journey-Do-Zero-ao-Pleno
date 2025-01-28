/* A tarefa proposta é desenvolver um programa em Rust que leia um número inteiro e 
exiba a sua tabuada. O programa deve começar lendo um número inteiro do usuário. 
Depois, deve-se criar um loop que percorra os números inteiros de 1 a 10. 
Para cada número, deve-se calcular o resultado da multiplicação*/

use std::io;

fn tabuada(numero:i32){
    let range = 1..=10;

    println!("Tabuado do {}:", numero);
    for i in range{
        let multiplicacao = i * numero;
        
        println!("{} x {} = {}",numero, i, multiplicacao);
    }
}


fn main(){
    let mut numero = String::new();
    io::stdin().read_line(&mut numero).expect("Erro! Entrada inválida");
    let numero_int = numero.trim().parse::<i32>().unwrap();

    tabuada(numero_int);

    println!("\n");
    tabuada(8);
    
}