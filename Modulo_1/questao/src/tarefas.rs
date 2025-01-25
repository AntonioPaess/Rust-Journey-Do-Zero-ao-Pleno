use std::io;



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
    }

    println!("A quantidade de alunos em recuperação é: {}.", soma_rec);
    println!("E a quantidade de alunos aprovados é: {}",soma_aprov);



    // Tarefa 3 - Maior Divisor Comum usando While

    
    
}