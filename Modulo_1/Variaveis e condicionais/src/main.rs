use  std::io; // Importando a biblioteca de entrada e saida


fn main() {

    let name: &str = "Antônio"; // Variaveis sao imutaveis em rust por padrão

    // para que a variavel seja mutavel você tem que usar o modificador mut
    let mut age: u64= 22;
    age += 1; 
    
    println!("Hello, {}!", name);
    print!("You are {} years old\n", age);

    //Inteiros
    let x = 23; // por padrao no rust o inteiro é i32(32 bits)
    // para modificar isso basta colocar x: i64 para que ele seja um inteiro de 64 bits e tenha mais capacidade de armazenamento
    
    // em rust você também pode proibir o uso de numeros negativos usando o modificador u de unsigned
    let idade: u64 = 18; //idade é um valor que nao pode ser negativo.

    //Floats
    let pi: f64 = 3.14159265359; // por padrao o float é f64(64 bits)
    // caso eu queira formatar para 2 casas decimais eu posso usar o modificador .2 para que ele tenha 2 casas decimais
    let pi2: f64 = 3.14159265359;

    print!("Pi: {}\n", pi);
    print!("Pi Formatado: {:.2}\n", pi2);
    
    //Booleanos

    let maior_idade: bool = true; // true ou false
    let  pode_dirigir: bool;

    //fluxo de controle(condicionais)

    if idade < 18 {
        pode_dirigir = false;
    }
    else{
        pode_dirigir = true;
    }

    if pode_dirigir {
        print!("Você pode dirigir\n");
    }
    else{
        print!("Você não pode dirigir\n");
    }

    if age > idade{
        println!("{} > {}\n", age, idade);
    }
    else if age == idade{
        println!("{} = {}\n", age, idade);
    }
    else{
        println!("{} < {}\n", age, idade);
    }


    if idade > 18 && age > 18{
        print!("Você pode dirigir\n");
    }
    else{
        print!("Você não pode dirigir\n");
    }

    if idade > 18 || age > 18{
        print!("Você pode dirigir\n");
    }
    else{
        print!("Você não pode dirigir\n");
    }
    



   



}









