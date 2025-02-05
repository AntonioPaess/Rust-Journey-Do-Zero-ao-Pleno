enum OperacaoBancaria {
    Deposito(f32),
    Saque(f32),
    Saldo,
}

fn processar_operacao(saldo: f32, metodo: OperacaoBancaria) -> f32 {
    match metodo {
        OperacaoBancaria::Deposito(valor) => {
            println!("Depósito de {} reais realizado.", valor);
            saldo + valor
        }
        OperacaoBancaria::Saque(valor) => {
            if saldo >= valor {
                println!("Saque de {} reais realizado.", valor);
                saldo - valor
            } else {
                println!("Erro: Saldo insuficiente! Seu saldo atual é {} reais.", saldo);
                saldo
            }
        }
        OperacaoBancaria::Saldo => {
            println!("Seu saldo atual é: {} reais.", saldo);
            saldo
        }
    }
}

fn main() {
    let mut saldo = 1000.0;

    saldo = processar_operacao(saldo, OperacaoBancaria::Deposito(400.0));
    saldo = processar_operacao(saldo, OperacaoBancaria::Saque(200.0));
    saldo = processar_operacao(saldo, OperacaoBancaria::Saque(1500.0)); 
    saldo = processar_operacao(saldo, OperacaoBancaria::Saldo);

    println!("Saldo final: {}", saldo);
}
