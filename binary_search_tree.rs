use std::cmp::Ordering;
use std::fmt::{self, Display, Formatter};

// Definição de um nó da árvore binária de busca
#[derive(Debug)]
struct Node {
    valor: i32,
    esquerda: Option<Box<Node>>,
    direita: Option<Box<Node>>,
}

impl Node {
    // Cria um novo nó com o valor fornecido
    fn novo(valor: i32) -> Self {
        Node {
            valor,
            esquerda: None,
            direita: None,
        }
    }

    // Insere um valor na árvore binária de busca
    fn inserir(&mut self, valor: i32) {
        match valor.cmp(&self.valor) {
            Ordering::Less => {
                if let Some(ref mut no_esquerda) = self.esquerda {
                    no_esquerda.inserir(valor);
                } else {
                    self.esquerda = Some(Box::new(Node::novo(valor)));
                }
            }
            Ordering::Greater => {
                if let Some(ref mut no_direita) = self.direita {
                    no_direita.inserir(valor);
                } else {
                    self.direita = Some(Box::new(Node::novo(valor)));
                }
            }
            Ordering::Equal => {
                println!("Valor {} já existe na árvore.", valor);
            }
        }
    }

    // Realiza uma busca na árvore binária
    fn buscar(&self, valor: i32) -> bool {
        match valor.cmp(&self.valor) {
            Ordering::Less => match &self.esquerda {
                Some(no_esquerda) => no_esquerda.buscar(valor),
                None => false,
            },
            Ordering::Greater => match &self.direita {
                Some(no_direita) => no_direita.buscar(valor),
                None => false,
            },
            Ordering::Equal => true,
        }
    }
}

// Implementação da árvore binária de busca
#[derive(Debug)]
struct ArvoreBinaria {
    raiz: Option<Box<Node>>,
}

impl ArvoreBinaria {
    // Cria uma nova árvore vazia
    fn nova() -> Self {
        ArvoreBinaria { raiz: None }
    }

    // Insere um valor na árvore
    fn inserir(&mut self, valor: i32) {
        match self.raiz {
            Some(ref mut no_raiz) => no_raiz.inserir(valor),
            None => self.raiz = Some(Box::new(Node::novo(valor))),
        }
    }

    // Busca por um valor na árvore
    fn buscar(&self, valor: i32) -> bool {
        match &self.raiz {
            Some(no_raiz) => no_raiz.buscar(valor),
            None => false,
        }
    }
}


// Implementa a exibição da árvore em ordem
impl Display for ArvoreBinaria {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        fn escrever(f: &mut Formatter<'_>, no: &Option<Box<Node>>, nivel: usize) -> fmt::Result {
            if let Some(n) = no {
                escrever(f, &n.esquerda, nivel + 1)?;
                writeln!(f, "{:indent$}{}", "", n.valor, indent = nivel * 4)?;
                escrever(f, &n.direita, nivel + 1)?;
            }
            Ok(())
        }

        escrever(f, &self.raiz, 0)
    }
}

fn main() {
    let mut arvore = ArvoreBinaria::nova();

    // Insere elementos na árvore
    arvore.inserir(50);
    arvore.inserir(30);
    arvore.inserir(70);
    arvore.inserir(20);
    arvore.inserir(40);
    arvore.inserir(60);
    arvore.inserir(80);

    // Exibe a árvore em ordem
    println!("Árvore Binária de Busca (em ordem):");
    println!("{}", arvore);

    // Realiza uma busca na árvore
    let valor_para_buscar = 40;
    if arvore.buscar(valor_para_buscar) {
        println!("Valor {} encontrado na árvore.", valor_para_buscar);
    } else {
        println!("Valor {} não encontrado na árvore.", valor_para_buscar);
    }
}
