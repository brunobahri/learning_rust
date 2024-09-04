fn busca_binaria(lista: &[i32], alvo: i32) -> Option<usize> {
  let mut inicio = 0;
  let mut fim = lista.len();

  while inicio < fim {
      let meio = (inicio + fim) / 2;
      if lista[meio] == alvo {
          return Some(meio);
      } else if lista[meio] < alvo {
          inicio = meio + 1;
      } else {
          fim = meio;
      }
  }

  
  None
}

fn main() {
  let lista = [1, 3, 5, 7, 9, 11, 13, 15, 17, 19];
  let alvo = 7;

  match busca_binaria(&lista, alvo) {
      Some(indice) => println!("Elemento {} encontrado no índice {}.", alvo, indice),
      None => println!("Elemento {} não encontrado na lista.", alvo),
  }
}

