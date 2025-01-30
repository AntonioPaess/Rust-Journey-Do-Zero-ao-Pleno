fn eh_palindromo(x: i32) -> bool{
    if x < 0{
        return false;
    }
    let string: String = x.to_string();
    let string_invertida: String = string.chars().rev().collect();

    string == string_invertida 
}

fn main(){
    let num1 = 010;
    let num2 = -121;
    let num3 = 101;

    println!("É {} um palíndromo? {}", num1, eh_palindromo(num1));
    println!("É {} um palíndromo? {}", num2, eh_palindromo(num2));
    println!("É {} um palíndromo? {}", num3, eh_palindromo(num3));
}