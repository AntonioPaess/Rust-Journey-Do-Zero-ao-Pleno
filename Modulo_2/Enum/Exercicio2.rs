/* Crie um enum StatusEncomenda com as variantes EmTransito, Entregue, Atrasado e Cancelado.
 Em seguida, implemente uma função verificar_status que recebe um StatusEncomenda 
e imprime uma mensagem diferente dependendo do status da encomenda.*/

enum StatusEncomenda{
    EmTransito,
    Entregue,
    Atrasado,
    Cancelado
}

fn verificar_status(status:StatusEncomenda){
    match status{
        StatusEncomenda::EmTransito => println!("Sua encomenda está em trânsito."),
        StatusEncomenda::Atrasado => println!("Sua encomenda está Atrasada. Pedimos desculpas pelo transtorno."),
        StatusEncomenda::Cancelado => println!("A entrega da sua encomenda foi cancelada."),
        StatusEncomenda::Entregue => println!("Sua encomenda está foi entregue.")
    }
}

fn main(){
    verificar_status(StatusEncomenda::Atrasado);
    verificar_status(StatusEncomenda::EmTransito);
    verificar_status(StatusEncomenda::Cancelado);
    verificar_status(StatusEncomenda::Entregue);
}