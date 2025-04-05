# Amnesia Quicksort

## Autor
- [Felipe Tertuliano](https://github.com/felipe-tertuliano)

## Introdução

Projeto em rust desenvolvido para gerar traces para o simulador Amnesia.

## Uso

Para executar o projeto primeiramente se deve [instalar Rust e Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) no ambiente. Após a instalação na mesma pasta em que o projeto foi baixado executar o seguinte comando no terminal:
```console
cargo run
```

## Metodologia

Inicialmente é preciso entender o funcionamento do simulador Amnesia e a proposta associada ao trace gerado pelo código. O simulador Amnesia tem como objetivo simular, de forma visual, o funcionamento da memória em um processador. Para tanto, ele utiliza um arquivo de trace que representa a execução de um programa. Entretanto, o formato do trace empregado pelo Amnesia difere dos padrões convencionais.

Conforme descrito por Carlos Emilio de Andrade Cacho (2015), o arquivo trace ou arquivo de trace deve obedecer às seguintes orientações:[^1]

> Cada linha deste arquivo é composta de uma dupla: rótulo (decimal) e endereço (hexadecimal). Qualquer outra informação é vista como um comentário. Um exemplo pode ser visto na Figura 11. Os rótulos possíveis são:
> 
> - **0:** leitura de dados;
> - **1:** gravação de dados;
> - **2:** busca de instrução;
> - **3:** registro escape (tratado como tipo de acesso desconhecido);
> - **4:** registro escape (operação de cache flush).

Para a proposta apresentada, serão usados somente os rótulos de leitura e gravação de dados (0 e 1, respectivamente), isso porque será analisado o impacto do número de acessos a memória, o que independe das outras funcionalidades. E partindo dessas premissas, o objetivo é desenvolver um código capaz de executar o algoritmo Quicksort em três cenários (melhor caso, caso médio e pior caso) e imprimir os traces compatíveis com a execução e o simulador Amnesia.

### Endereços

Para simular a alocação real dos endereços no trace, são utilizadas duas variáveis globais de controle: `ADRS_HASH` e `PC`. A variável `ADRS_HASH` representa uma tabela hash onde as chaves são os endereços reais das variáveis utilizadas no código e os valores são endereços falsos (menores) que representam essas chaves no trace. Já `PC` é um contador de 8 bits usado para gerar os valores de `ADRS_HASH`. Para cada nova chave, o valor atribuído é igual ao conteúdo atual de `PC` e, em seguida, `PC` é incrementado em 1, evitando assim a repetição de dados.

```rust
let pc = state::get(&PC);
let tmp = pc + 1;
adrs_hash.insert(value, tmp);
state::set(&ADRS_HASH, adrs_hash);
state::set(&PC, tmp);
new_value = tmp;
```

O código acima está disponível em [GitHub](https://github.com/felipe-tertuliano/amnesia_quicksort/tree/main/src/tracer/mod.rs) (linhas 27 a 35).

### Mapeamento do Trace

O mapeamento do trace é realizado por meio da função `rw`, que recebe dois vetores de endereços: um para leituras e outro para escritas. Dessa forma, é possível replicar a execução de linhas de código por meio de traces. Junto disso, a função `adrs` é utilizada para converter o ponteiro de uma variável em um valor legível para `ADRS_HASH`. A Tabela abaixo ilustra alguns exemplos de uso das funções.

| Linha de Código   | Equivalente com `rw` e `adrs`   |
| ----------------- | ------------------------------- |
| `if a == true { ` | `rw(&[adrs(&a)], &[]);`         |
| `a = 2`           | `rw(&[], &[adrs(&a)]);`         |
| `a = b;`          | `rw(&[adrs(&b)], &[adrs(&a)]);` |

As funções são implementadas da seguinte forma:

```rust
pub fn adrs<T>(value: &T) -> usize {s
    return value as *const T as usize;
}

pub fn rw(r: &[usize], w: &[usize]) {
    for e in r {
        trace(TraceTypeEnum::READ, *e);
    }
    for e in w {
        trace(TraceTypeEnum::WRITE, *e);
    }
}
```

Conforme demonstrado, a função `rw` itera pelos vetores de leitura e escrita, invocando a função `trace` com o tipo de acesso adequado para cada endereço. O código acima está disponível em [GitHub](https://github.com/felipe-tertuliano/amnesia_quicksort/tree/main/src/tracer/mod.rs) (linhas 41 a 52).

### Implementação do Quicksort

O algoritmo Quicksort foi implementado na função `trace_quicksort`. Para cada operação de leitura ou escrita realizada sobre uma variável, há uma chamada correspondente que mapeia essa ação no trace. Na seleção do pivô, é escolhido o valor no meio do conjunto a ser ordenado, e a troca de valores entre elementos do vetor é realizada pela função `trace_swap`, também devidamente mapeada.

```rust
fn trace_swap(arr: &mut Vec<u8>, a: &isize, b: &isize) {
	let tmp = arr[*a as usize];
	rw(&[adrs(a), adrs(arr)], &[adrs(&tmp)]);

	arr[*a as usize] = arr[*b as usize];
	rw(&[adrs(b), adrs(arr), adrs(a)], &[adrs(&arr)]);

	arr[*b as usize] = tmp;
	rw(&[adrs(&tmp)], &[adrs(&arr)]);
}

pub fn trace_quicksort(arr: &mut Vec<u8>, low: &isize, high: &isize) {
	rw(&[adrs(low), adrs(high)], &[]);
	if low < high {
		let mut tmp = *high - *low;
		rw(&[adrs(high), adrs(low)], &[adrs(&tmp)]);
		tmp = tmp / 2;
		rw(&[adrs(&tmp)], &[adrs(&tmp)]);

		let mid = *low + tmp;
		rw(&[adrs(&low), adrs(&tmp)], &[adrs(&mid)]);
		let pivot = arr[mid as usize];
		rw(&[adrs(&mid), adrs(&arr)], &[adrs(&pivot)]);

		let mut i = *low;
		rw(&[adrs(&low)], &[adrs(&i)]);
		let mut j = *high;
		rw(&[adrs(&high)], &[adrs(&j)]);

		while i <= j {
			rw(&[adrs(&i), adrs(&j)], &[]);

			while arr[i as usize] < pivot {
				rw(&[adrs(&i), adrs(&arr), adrs(&pivot)], &[]);
				i = i + 1;
				rw(&[adrs(&i)], &[adrs(&i)]);
			}
			rw(&[adrs(&i), adrs(&arr), adrs(&pivot)], &[]); // `while` exit validation

			while arr[j as usize] > pivot {
				rw(&[adrs(&j), adrs(&arr), adrs(&pivot)], &[]);
				j = j - 1;
				rw(&[adrs(&j)], &[adrs(&j)]);
			}
			rw(&[adrs(&j), adrs(&arr), adrs(&pivot)], &[]); // `while` exit validation

			rw(&[adrs(&i), adrs(&j)], &[]);
			if i <= j {
				trace_swap(arr, &i, &j);
				i = i + 1;
				rw(&[adrs(&i)], &[adrs(&i)]);
				j = j - 1;
				rw(&[adrs(&j)], &[adrs(&j)]);
			}
		}
		rw(&[adrs(&i), adrs(&j)], &[]); // `while` exit validation

		rw(&[adrs(&i), adrs(&high)], &[]);
		if i < *high {
			trace_quicksort(arr, &i, high);
		}

		rw(&[adrs(&low), adrs(&j)], &[]);
		if *low < j {
			trace_quicksort(arr, low, &j);
		}
	}
}
```

O código acima está disponível em [GitHub](https://github.com/felipe-tertuliano/amnesia_quicksort/tree/main/src/tracer/quicksort.rs) (linhas 3 a 70).

### Traces Gerados

Foram simulados três cenários distintos para a execução do Quicksort, correspondentes ao melhor caso, caso médio e pior caso. Em cada cenário, um vetor representativo do ambiente é processado pela função `trace_quicksort` e o trace resultante é impresso no terminal. Para facilitar a análise, os traces gerados foram salvos separadamente em arquivos de texto.

#### Cenários de Teste e Vetores Utilizados

| Caso        | Vetor              |
| ----------- | ------------------ |
| Melhor Caso | [1, 2, 3, 4, 5, 6] |
| Caso Médio  | [6, 2, 4, 3, 5, 1] |
| Pior Caso   | [6, 5, 4, 3, 2, 1] |

A seguir, a implementação dos cenários de teste:

```rust
fn best_case_scenario() {
    println!("Best Case Scenario");
    let mut arr = vec![1, 2, 3, 4, 5, 6];
    let high = (arr.len() - 1) as isize;
    trace_quicksort(&mut arr, &0, &high);
    tracer::reset_pc();
}

fn average_case_scenario() {
    println!("Average Case Scenario");
    let mut arr = vec![6, 2, 4, 3, 5, 1];
    let high = (arr.len() - 1) as isize;
    trace_quicksort(&mut arr, &0, &high);
    tracer::reset_pc();
}

fn worst_case_scenario() {
    println!("Worst Case Scenario");
    let mut arr = vec![6, 5, 4, 3, 2, 1];
    let high = (arr.len() - 1) as isize;
    trace_quicksort(&mut arr, &0, &high);
    tracer::reset_pc();
}

fn main() {
    best_case_scenario();
    average_case_scenario();
    worst_case_scenario();
}
```

O código acima está disponível em [GitHub](https://github.com/felipe-tertuliano/amnesia_quicksort/tree/main/src/main.rs) (linhas 5 a 33).

[^1]: Carlos Emilio de Andrade Cacho, 2015. *Amnesia, Tutorial técnico do módulo memória virtual. Arquivos de trace.*