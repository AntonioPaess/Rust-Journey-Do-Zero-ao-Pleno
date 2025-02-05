/* Crie um enum MetodoPagamento com as variantes Credito(f32), Debito(f32), Pix(f32), e Boleto. 
Em seguida, crie uma função chamada processar_pagamento que recebe um MetodoPagamento e 
imprime a forma de pagamento e o valor (se aplicável). */

enum MetodoPagamento{
    Credito(f32),
    Debito(f32),
    Pix(f32),
    Boleto
}

fn processar_pagamento(pagamento:MetodoPagamento){
    match pagamento{
        MetodoPagamento::Credito(x) => println!("Pagamento de {} reais realizado via cartão de crédito", x),
        MetodoPagamento::Pix(x) => println!("Pagamento de {} reais realizado via Pix", x),
        MetodoPagamento::Debito(x) => println!("Pagamento de {} reais realizado via cartão de Débito", x),
        MetodoPagamento::Boleto => println!("Pagamento realizado via Boleto"),
    }
}

fn main(){
    processar_pagamento(MetodoPagamento::Pix(150.0));
    processar_pagamento(MetodoPagamento::Credito(1599.99));
    processar_pagamento(MetodoPagamento::Debito(32.5));
    processar_pagamento(MetodoPagamento::Boleto);
}