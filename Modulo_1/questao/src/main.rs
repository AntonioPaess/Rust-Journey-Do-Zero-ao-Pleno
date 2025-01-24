 use std::io; //Biblioteca de entrada e saida
 
 //Condicionais com entrada do usuario


    /*Quais os objetivos:
    1. transformar de string para inteiro
 */

 fn convert_to_int(data_input: &String) -> i32{
    let x = data_input.trim().parse::<i32>().unwrap();
    return x;
 } 


fn main() {
let mut number1 = String::new();
println!("Digite o primeiro número: ");
io::stdin().read_line(&mut number1).expect("Falha ao ler a linha");
    
let mut number2 = String::new();
println!("Digite o segundo número: ");
io::stdin().read_line(&mut number2).expect("Falha ao ler a linha");

if convert_to_int(&number1) > convert_to_int(&number2){
    println!("O número {} é maior que o {}", number1, number2);
}
else{
    println!("O número {} é menor ou igual que o {}", number1, number2);
}


/* Desafio 1: Comparar Três Números
Modifique o código para solicitar três números do usuário e 
determine qual é o maior dos três. 
Exiba uma mensagem indicando qual número é o maior. */

let mut numero: String = String::new();
let mut numero2: String = String::new();
let mut numero3: String = String::new();
let mut maior:i32;

println!("Digite o primeiro número: ");
io::stdin().read_line(&mut numero).expect("Erro ao ler a linha");

println!("Digite o segundo número: ");
io::stdin().read_line(&mut numero2).expect("Erro ao ler a linha");

println!("Digite o terceiro número: ");
io::stdin().read_line(&mut numero3).expect("Erro ao ler a linha");

if convert_to_int(&numero) > convert_to_int(&numero2){
    maior = convert_to_int(&numero);
}
else{
    maior = convert_to_int(&numero2);
}

if maior < convert_to_int(&numero3){
    maior = convert_to_int(&numero3);
}

println!("Entre os números {}, {} e {}. \nO maior número é: {}",numero, numero2, numero3, maior);

/*Desafio 2: Soma e Média de Dois Números
Modifique o código para calcular a soma e a 
média dos dois números inseridos pelo usuário. 
Exiba a soma e a média no console. */

let mut matematica: String = String::new();
let mut portugues: String = String::new();
let  soma: i32;
let  media: f32;

println!("Digite a nota de Matemática: ");
io::stdin().read_line(&mut matematica).expect("Erro ao ler a linha");

println!("Digite a nota de Português: ");
io::stdin().read_line(&mut portugues).expect("Erro ao ler linha");

let nota1 = convert_to_int(&matematica);
let nota2 = convert_to_int(&portugues);

soma = nota1 + nota2;

media = soma as f32 / 2.0;

println!("A soma das notas é: {}", soma); 
println!("A média das notas é: {}", media);


/*Desafio 3: Soma de N Números
Solicite ao usuário que insira a quantidade de números que ele deseja somar. 
Em seguida, use um loop for para ler esses números, somá-los e exibir a soma total. */

let mut quantidade: String = String::new();

println!("Digite a quantidade de numeros que deseja somar:\n");
io::stdin().read_line(&mut quantidade).expect("Falha ao ler linha");

let quantidade = convert_to_int(&quantidade);
let mut soma: i32 = 0;

for i in 0..quantidade{
    let mut numero: String = String::new();
    println!("Digite o número {}:", i+1);
    io::stdin().read_line(&mut numero).expect("Falha ao ler linha");
    soma += convert_to_int(&numero);
    
}

println!("A soma dos números é: {}", soma);

/* Desafio 2: Média de N Números com Validação de Entrada
Solicite ao usuário que insira a quantidade de números que ele deseja calcular a média. 
Use um loop while para garantir que o usuário insira apenas números válidos. 
Calcule a média e exiba o resultado. */

let mut quantidade: String = String::new();

println!("Digite a quantidade de numeros que deseja calcular a média:\n");
io::stdin().read_line(&mut quantidade).expect("Falha ao ler linha");

let  quantidade_total = convert_to_int(&quantidade);
let mut quantidade = convert_to_int(&quantidade);
let mut soma: i32 = 0;
let media: f32;

while quantidade > 0{
    
    let mut numero: String = String::new();
    
    println!("Digite o número:");
    io::stdin().read_line(&mut numero).expect("Falha ao ler linha");

    let numero = convert_to_int(&numero);

    if numero >= 0{
        soma += numero;
        quantidade -= 1;
    }
    else {
        println!("Número inválido, digite um número maior que 0");
    }

    
    
} 
media = soma as f32 / quantidade_total as f32;
println!("A média dos números é: {}", media);
    
}

