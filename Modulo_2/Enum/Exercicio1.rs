/* Crie um enum chamado Direcao com as variantes Cima, Baixo, Esquerda e Direita. 
Depois, implemente uma função chamada mover_personagem que recebe um valor do tipo 
Direcao e imprime uma mensagem informando para onde o personagem foi movido. */


enum Direcao{
    Cima, 
    Baixo,
    Esquerda,
    Direita
}

fn mover_personagem(movimento:Direcao){
    match movimento{
        Direcao::Cima => println!("O personagem se moveu para cima"),
        Direcao::Baixo => println!("O personagem se moveu para baixo"),
        Direcao::Esquerda => println!("O personagem se moveu para esquerda"),
        Direcao::Direita => println!("O personagem se moveu para direita")
    }
}


fn main(){
    mover_personagem(Direcao::Cima);
    mover_personagem(Direcao::Baixo);
    mover_personagem(Direcao::Esquerda);
    mover_personagem(Direcao::Direita);
}