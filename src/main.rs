use std::io;

fn main() {
    println!("Insira o primeiro numero: ");
    let mut primeiro_numero = String::new();

    io::stdin()
    .read_line(&mut primeiro_numero)
    .expect("Algo aconteceu para a linha nao ser lida!");

    print!("O primeiro numero e: {primeiro_numero}");

    println!("Insira o segundo numero: ");
    let mut segundo_numero: String = String::new();

    io::stdin()
    .read_line(&mut segundo_numero)
    .expect("Algo aconteceu para a linha não ser lida!");

    let primeiro: f64 = primeiro_numero.trim().parse().expect("Ocorreu um erro na conversao.");
    let segundo: f64 = segundo_numero.trim().parse().expect("Ocorre um erro na conversao.");

    println!("O segundo numero e: {segundo_numero}");

    println!("Agora escolha a equacao realizada:");
    println!("1. +");
    println!("2. -");
    println!("3. *");
    println!("4. /");
    println!("5. %");

    let mut equacao: String = String::new();

    io::stdin()
    .read_line(&mut equacao)
    .expect("Ocorreu um erro ao escolher uma equacao!");

    let equacao = equacao.trim();

    match equacao {
        "1" => print!("Resultado final: {}", primeiro + segundo),
        "2" => print!("Resultado final: {}", primeiro - segundo),
        "3" => print!("Resultado final: {}", primeiro * segundo),
        "4" => print!("Resultado final: {}", primeiro / segundo),
        "5" => print!("Resultado final: {}", primeiro % segundo),
        _ => print!("Nenhuma equacao foi encontrada!")
    }
}