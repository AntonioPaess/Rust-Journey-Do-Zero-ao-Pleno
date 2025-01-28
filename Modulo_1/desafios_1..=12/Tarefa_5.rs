/* Esta tarefa requer que um programa em Rust seja desenvolvido para ler uma sequência de números reais 
e exibir a soma dos números pares. Para isso, o programa deve começar lendo os números fornecidos pelo usuário. 
Em seguida, deve-se criar um loop que percorra a sequência de números e calcule a soma dos números pares.
 Por fim, deve-se imprimir a soma na tela e testar o programa 
 para garantir que ele está funcionando corretamente. */

use std::io;

 fn soma_pares(lista: Vec<f64>) -> f64{
    let mut soma:f64 = 0.0;

    for i in lista{
        if i % 2.0 == 0.0{
            soma += i;
        }
    }
    soma
}



fn main(){
    let mut entrada = String::new();

    io::stdin().read_line(&mut entrada).expect("Falha na entrada");
    let lista:Vec<f64> = entrada
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<f64>().expect("Inserir apenas números reais"))
        .collect();
    
    println!("O resultado da soma dos números pares é: {}", soma_pares(lista));



}