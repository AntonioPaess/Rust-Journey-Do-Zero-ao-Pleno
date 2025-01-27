use std::io;

fn dobro(numero: i32) -> i32 {
    return numero * 2;
}

fn comparacao(valor_a:i32, valor_b: i32) -> i32{
    if valor_a >= valor_b{
        return valor_a;
    }
    else{
        return valor_b;
    }
}


fn fatorial(valor:i32) -> i32{
    let mut fatorial = 1;
    let range = 1..=valor;

    for i in range{
        fatorial *= i;
    } 
    return fatorial;
}



fn main(){
    
    // Tarefa 1 - Fatorial usando Loop While 
    
    let mut entrada = String::new();
    
    io::stdin().read_line(&mut entrada).expect("Falha na leitura da entrada");
    
    let mut entrada_convertida = entrada.trim().parse::<i32>().unwrap();
    
    let mut multiplicacao = 1;

    while entrada_convertida > 1{

        multiplicacao *= entrada_convertida;
        entrada_convertida -= 1;
    }

    println!("O fatorial é: {}", multiplicacao);


    // Tarefa 2 - Quantos alunos ficaram em recuperação
    // media >= 7 


    let mut quantidade_alunos = String::new();
    let mut boletim = String::new();

    
    io::stdin().read_line(&mut quantidade_alunos).expect("Falha ao ler entrada do usuário");

    
    io::stdin().read_line(&mut boletim).expect("Falha ao ler entrada do usuário");

    let mut quantidade_alunos_int = quantidade_alunos.trim().parse::<i32>().unwrap();
    let boletim_int = boletim.trim().parse::<i32>().unwrap();
    let mut soma_rec = 0;
    let mut soma_aprov = 0;

    while quantidade_alunos_int > 0 {
        let mut soma = 0;

        println!("Digite as {} notas do aluno:", boletim_int);

        let mut contador_notas = 0;

        while contador_notas < boletim_int {
            let mut nota = String::new();
            io::stdin().read_line(&mut nota).expect("Falha ao ler entrada do usuário");

            let nota_int: i32 = nota
                .trim()
                .parse()
                .expect("Por favor, insira uma nota válida.");
            soma += nota_int;
            contador_notas += 1;
        }

        let media = soma / boletim_int;
        println!("A média do aluno é: {}\n", media);

        if media >= 7 {
            println!("Aluno passou.\n");
            soma_aprov += 1;
        } else {
            println!("Aluno ficou em recuperação.\n");
            soma_rec += 1;
        }

        quantidade_alunos_int -= 1; 
        
        println!("A quantidade de alunos em recuperação é: {}.", soma_rec);
        println!("E a quantidade de alunos aprovados é: {}",soma_aprov);
        }



    // Tarefa 3 - Maior Divisor Comum usando While

    let mut divisor_a = 400; 
    let mut divisor_b = 320; 
    let mut resto;

    while divisor_b != 0 {
        resto = divisor_a % divisor_b; 
        divisor_a = divisor_b;        
        divisor_b = resto;            
    }

    println!("O MDC é: {}", divisor_a);

    // Tarefa 4 - Criar uma função que retorna o dobro de um número inteiro

    let valor:i32 = 2;
    let resultado:i32;

    resultado = dobro(valor);
    println!("O dobro do valor escolhido é: {}", resultado);

    // Tarefa 5 - Qual o maior número? Faça uma função de comparação entre dois números

    let valor_a = 100;
    let valor_b =  25;

    let resposta = comparacao(valor_a, valor_b);

    println!("O maior valor entre {} e {} é: {}", valor_a, valor_b, resposta);


    // Tarefa 6 - Faça uma função que realiza a fotoração de cada item em um vetor

    let lista = vec![1,2,3,4,5];
    

    for i in lista{
        println!("O fatorial do número {} é: {}", i, fatorial(i));
    }

    // Tarefa 7 - Crie um exemplo de Closure
    let mut entrada_usuario = String::new();

    io::stdin().read_line(&mut entrada_usuario).expect("Erro ao ler entrada");
    let valor_int = entrada_usuario.trim().parse::<i32>().unwrap();
    
    let quadrado = |x:i32| -> i32 {x * x};
    println!("O quadrado de {} é: {}",valor_int, quadrado(valor_int))


    
}