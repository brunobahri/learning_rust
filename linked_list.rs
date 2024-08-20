use std::rc::Rc;
use std::cell::RefCell;

// Definição de um nó da lista encadeada
#[derive(Debug)]
struct Node {
    valor: i32,
    proximo: Option<Rc<RefCell<Node>>>,
}

// Definição da lista encadeada
#[derive(Debug)]
struct ListaEncadeada {
    cabeca: Option<Rc<RefCell<Node>>>,
}

impl ListaEncadeada {
    // Cria uma nova lista vazia
    fn nova() -> Self {
        ListaEncadeada { cabeca: None }
    }

    // Adiciona um novo nó ao início da lista
    fn adicionar(&mut self, valor: i32) {
        let novo_no = Rc::new(RefCell::new(Node {
            valor,
            proximo: self.cabeca.take(),
        }));
        self.cabeca = Some(novo_no);
    }

    // Exibe os elementos da lista
    fn exibir(&self) {
        let mut no_atual = self.cabeca.clone();
        while let Some(no) = no_atual {
            print!("{} -> ", no.borrow().valor);
            no_atual = no.borrow().proximo.clone();
        }
        println!("None");
    }
}

fn main() {
    let mut lista = ListaEncadeada::nova();

    // Adiciona elementos na lista
    lista.adicionar(10);
    lista.adicionar(20);
    lista.adicionar(30);

    // Exibe os elementos da lista
    lista.exibir();
}
