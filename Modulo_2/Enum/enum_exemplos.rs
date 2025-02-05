

#[derive(Debug)]
enum Directions{
    Up,
    Down,
    Left,
    Right
}

#[derive(Debug)]
enum CarType{
    Fiat,
    Ford,
    Renault
}

enum Pagamento{
    Dinheiro(f32),
    Credito(bool, f32),
    Pix(bool, f32),
    Debito(bool, f32)
}


fn nacionalidade_carro(carro: CarType){
    match carro{
        CarType::Fiat => println!("Esse carro é italiano"),
        CarType::Ford => println!("Esse carro é Americano"),
        CarType::Renault => println!("Esse carro é Frânces")
        
    }
}

// Usando essa funçao para forcar o erro e ver que o match avisa do novo metodo no enum pagamento
fn metodo_de_pagamento(pessoa:Pagamento){
    match pessoa{
        Pagamento::Dinheiro(f) => println!("A pessoa escolheu pagar {} reais em dinheiro", f),
        Pagamento::Pix(true, x) => println!("A pessoa escolheu pagar {} reais em Pix", x),
        Pagamento::Credito(true,y) => println!("A pessoa escolheu pagar com o Cartão de Crédito"),
        Pagamento::Credito(false,_) => println!("O cartao de crédito foi recusado"),
        Pagamento::Debito(false, z) => println!("A pessoa escolheu pagar com o Cartão de Débito"),
        _ => {}

    }
}



fn main(){
    let pessoa_pagamento:Pagamento = Pagamento::Pix(true, 500f32);
    let player:Directions = Directions::Right;
    let carro:CarType = CarType::Renault;

    nacionalidade_carro(carro);
    nacionalidade_carro(CarType::Fiat);
    nacionalidade_carro(CarType::Ford);

    metodo_de_pagamento(pessoa_pagamento);
    

    match player{
        Directions::Up => println!("O jogador foi para cima"),
        Directions::Down => println!("O jogador foi para baixo"),
        Directions::Right => println!("O jogador foi para direita"),
        Directions::Left => println!("O jogador foi para esquerda")
    }
}