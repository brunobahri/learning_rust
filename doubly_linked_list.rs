use std::rc::Rc;
use std::cell::RefCell;

// Definição de um nó da lista duplamente encadeada
#[derive(Debug)]
struct Node {
    valor: i32,
    proximo: Option<Rc<RefCell<Node>>>,
    anterior: Option<Rc<RefCell<Node>>>,
}

// Definição da lista duplamente encadeada
#[derive(Debug)]
struct ListaDuplamenteEncadeada {
    cabeca: Option<Rc<RefCell<Node>>>,
    cauda: Option<Rc<RefCell<Node>>>,
}

impl ListaDuplamenteEncadeada {
    // Cria uma nova lista vazia
    fn nova() -> Self {
        ListaDuplamenteEncadeada { cabeca: None, cauda: None }
    }

    // Adiciona um novo nó ao final da lista
    fn adicionar(&mut self, valor: i32) {
        let novo_no = Rc::new(RefCell::new(Node {
            valor,
            proximo: None,
            anterior: self.cauda.clone(),
        }));

        match self.cauda.take() {
            Some(cauda_atual) => {
                cauda_atual.borrow_mut().proximo = Some(novo_no.clone());
                self.cauda = Some(novo_no);
            }
            None => {
                self.cabeca = Some(novo_no.clone());
                self.cauda = Some(novo_no);
            }
        }
    }

    // Exibe os elementos da lista do início ao fim
    fn exibir(&self) {
        let mut no_atual = self.cabeca.clone();
        while let Some(no) = no_atual {
            print!("{} -> ", no.borrow().valor);
            no_atual = no.borrow().proximo.clone();
        }
        println!("None");
    }

    // Exibe os elementos da lista do fim ao início
    fn exibir_reverso(&self) {
        let mut no_atual = self.cauda.clone();
        while let Some(no) = no_atual {
            print!("{} <- ", no.borrow().valor);
            no_atual = no.borrow().anterior.clone();
        }
        println!("None");
    }
}

fn main() {
    let mut lista = ListaDuplamenteEncadeada::nova();

    // Adiciona elementos na lista
    lista.adicionar(10);
    lista.adicionar(20);
    lista.adicionar(30);

    // Exibe os elementos da lista
    println!("Lista do início ao fim:");
    lista.exibir();

    // Exibe os elementos da lista do fim ao início
    println!("Lista do fim ao início:");
    lista.exibir_reverso();
}
