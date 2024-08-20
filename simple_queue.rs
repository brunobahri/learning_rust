use std::collections::VecDeque;

fn main() {
    // Cria uma fila usando VecDeque
    let mut fila: VecDeque<String> = VecDeque::new();

    // Adiciona elementos na fila (Enqueue)
    fila.push_back(String::from("Primeiro"));
    fila.push_back(String::from("Segundo"));
    fila.push_back(String::from("Terceiro"));

    // Exibe o estado atual da fila
    println!("Fila atual: {:?}", fila);

    // Remove o primeiro elemento da fila (Dequeue)
    if let Some(elemento) = fila.pop_front() {
        println!("Elemento removido: {}", elemento);
    }

    // Exibe o estado da fila após a remoção
    println!("Fila após remoção: {:?}", fila);

    // Verifica o próximo elemento da fila sem remover (Peek)
    if let Some(elemento) = fila.front() {
        println!("Próximo elemento na fila: {}", elemento);
    }
}
