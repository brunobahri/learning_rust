use std::io;

fn main() {
    // Exibe a mensagem pedindo o nome
    println!("Qual é o seu nome?");

    // Cria uma variável para armazenar o nome
    let mut nome = String::new();

    // Lê a entrada do usuário e armazena na variável 'nome'
    io::stdin()
        .read_line(&mut nome)
        .expect("Falha ao ler a entrada");

    // Remove a nova linha do final da entrada
    let nome = nome.trim();

    // Exibe a saudação personalizada
    println!("Olá, {}!", nome);
}



