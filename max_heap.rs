#[derive(Debug)]
struct MaxHeap {
    dados: Vec<i32>,
}

impl MaxHeap {
    // Cria um novo MaxHeap vazio
    fn novo() -> Self {
        MaxHeap { dados: Vec::new() }
    }

    // Insere um valor no MaxHeap
    fn inserir(&mut self, valor: i32) {
        self.dados.push(valor);
        self.subir(self.dados.len() - 1);
    }

    // Remove o maior valor (raiz) do MaxHeap
    fn extrair_max(&mut self) -> Option<i32> {
        if self.dados.is_empty() {
            return None;
        }

        let max = self.dados[0];
        let ultimo = self.dados.pop().unwrap();

        if !self.dados.is_empty() {
            self.dados[0] = ultimo;
            self.descer(0);
        }

        Some(max)
    }

    // Mantém a propriedade do Max Heap subindo o valor recém-adicionado
    fn subir(&mut self, mut indice: usize) {
        while indice > 0 {
            let pai = (indice - 1) / 2;
            if self.dados[indice] > self.dados[pai] {
                self.dados.swap(indice, pai);
                indice = pai;
            } else {
                break;
            }
        }
    }

    // Mantém a propriedade do Max Heap descendo o valor na raiz
    fn descer(&mut self, mut indice: usize) {
        let tamanho = self.dados.len();
        loop {
            let mut maior = indice;
            let esquerdo = 2 * indice + 1;
            let direito = 2 * indice + 2;

            if esquerdo < tamanho && self.dados[esquerdo] > self.dados[maior] {
                maior = esquerdo;
            }

            if direito < tamanho && self.dados[direito] > self.dados[maior] {
                maior = direito;
            }

            if maior == indice {
                break;
            }

            self.dados.swap(indice, maior);
            indice = maior;
        }
    }
}

fn main() {
    let mut heap = MaxHeap::novo();

    // Insere valores no Max Heap
    heap.inserir(10);
    heap.inserir(4);
    heap.inserir(9);
    heap.inserir(1);
    heap.inserir(7);
    heap.inserir(5);
    heap.inserir(3);

    println!("Max Heap: {:?}", heap);

    // Extrai o maior valor do Max Heap
    let max = heap.extrair_max();
    println!("Maior valor extraído: {:?}", max);
    println!("Max Heap após extração: {:?}", heap);
}
