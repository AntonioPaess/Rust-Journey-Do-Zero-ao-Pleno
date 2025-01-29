// Tarefa 1 - Crie uma função chamada count que receba um parâmetro inteiro chamado num.

fn count(num:i32){
    let mut contador = 1;
    for i in 1..=num{
        println!("O {}º valor é: {}", contador, i);
        contador +=1;
    }

}

fn count_down(mut num:i32){
    let mut contador_while = 1;

    while num >= 1{
        println!("O {}º valor é: {}", contador_while, num);
        contador_while += 1;
        num -= 1;

    }
}

fn main(){

    println!("Counting up to 10:");
    count(10);
    println!("Counting down from 10:");
    count_down(10);

}