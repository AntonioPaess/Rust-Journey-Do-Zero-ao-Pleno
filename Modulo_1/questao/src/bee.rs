use std::io;


fn converte_to_int(data_input: &String)-> i32{
    let x = data_input.trim().parse::<i32>().unwrap();
    return x;
}

 fn my_sort<T: PartialOrd + Copy>(lista: &mut [T]) {
    let tamanho = lista.len();

    for i in 0..tamanho {
        let mut min_index = i;

        for j in (i + 1)..tamanho {
            if lista[j] < lista[min_index] {
                min_index = j;       
            }
        }
        lista.swap(i, min_index);
    }
}

fn busca_binaria(list: &[i32], number: i32) -> Option<i32> {
    let mut lista_ordenada = list.to_vec();
    my_sort(&mut lista_ordenada);

    let mut baixo: i32 = 0;
    let mut alto: i32 = (lista_ordenada.len() - 1) as i32;

    while baixo <= alto {
        let meio = (baixo + alto) / 2;

        if lista_ordenada[meio as usize] == number {
            return Some(meio);
        } else if lista_ordenada[meio as usize] > number {
            alto = meio - 1;
        } else {
            baixo = meio + 1;
        }
    }
    None
}

fn main() {
    // Desafio 1930 BeeCrownd - Tomada
     let mut input = String::new();
    let quantidade:usize = 3;
    let mut soma:i32 = 0;
    io::stdin()
        .read_line(&mut input)
        .expect("Erro ao ler a linha");

    // Divida a entrada em partes e converta cada parte para i32
    let mut numbers: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|s| s.parse().expect("Erro ao converter para número"))
        .collect();

    // Imprima os números
    for i in 0.. quantidade {
        numbers[i] -= 1;
    }

    for number in numbers{
        soma += number;
    }

    println!("{}", soma); 

    // Desafio 2374 BeeCrownd - Pneu
     let mut input = String::new();
    let mut input2 = String::new();

    let valor1: i32;
    let valor2: i32;

    
    io::stdin().read_line(&mut input).expect("Erro ao ler a linha");
    io::stdin().read_line(&mut input2).expect("Erro ao ler a linha");

    valor1 = convert_to_int(&input);
    valor2 = convert_to_int(&input2);

    let diferenca = valor1 - valor2; 

    println!("{}", diferenca); */

     // Desafio 2413 BeeCrownd - Busca na Internet

    let mut input = String:: new();

    io::stdin().read_line(&mut input).expect("Erro ao ler a linha");    

    let link3:i32 = convert_to_int(&input);
    let link2:i32 = link3 * 2;
    let link1:i32 = link2 * 2;

    println!("{}", link1); */

    // Desafio 2006 BeeCrownd - Identificando o Chá

     let mut cha_escolhido = String::new();
    let mut input_respostas = String::new();

    io::stdin().read_line(&mut cha_escolhido).expect("Erro ao ler a linha");
    io::stdin().read_line(&mut input_respostas).expect("Erro ao ler a linha");

    let cha_certo = convert_to_int(&cha_escolhido);

    let mut respostas: Vec<i32> = input_respostas
    .trim()
    .split_whitespace()
    .map(|s| s
    .parse()
    .expect("Erro ao converter para número"))
    .collect::<Vec<i32>>();

    let mut quantidade_acertos = 0;
    
    if respostas.contains(&cha_certo){
        for resposta in respostas{
            if resposta == cha_certo{
                quantidade_acertos+=1;
            }
        }
    } 
    println!("{}", quantidade_acertos);*/ 

    // Desafio 1153 BeeCrownd - Fatorial Simples

     let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Falha ao ler entrada");

    let numero = input.trim().parse::<i32>().unwrap();

    let mut fatorial = 1;

    for i in 1..=numero{
        fatorial *= i;
    }

    println!("{}", fatorial);
*/

    // Busca Binária - Vou usar uma Def
    let minha_lista = vec![1, 2, 3, 4, 5, 7, 90, 32, 55, -1, -10, 0, 40, 28, 8];
    let resultado = busca_binaria(&minha_lista, 8);
    let teste = busca_binaria(&minha_lista, 32);
    println!("Está no índice: {:?}", resultado);
    println!("Está no índice: {:?}", teste);


    }


