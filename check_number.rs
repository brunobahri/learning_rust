use std::io;

fn main() {
    // Exibe a mensagem pedindo um número
    println!("Por favor, insira um número:");

    // Cria uma variável para armazenar o número
    let mut input = String::new();

    // Lê a entrada do usuário e armazena na variável 'input'
    io::stdin()
        .read_line(&mut input)
        .expect("Falha ao ler a entrada");

    // Converte a entrada para um número inteiro
    let numero: i32 = input.trim().parse().expect("Por favor, insira um número válido");

    // Verifica se o número é maior que 5
    if numero > 5 {
        println!("Maior que cinco");
    } else {
        println!("Menor ou igual a cinco");
    }
}

