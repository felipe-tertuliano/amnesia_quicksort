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

Inicialmente é preciso entender o funcionamento do simulador Amnesia e a proposta com o trace gerado no código. O simulador Amnesia tem como objetivo simular de forma visual o funcionamento da memória em um processador, para isso ele precisa de um arquivo de trace para representar um programa sendo executado, entretanto seu trace é diferente:

> Carlos Emilio de Andrade Cacho (2015). O arquivo trace ou arquivo de rastro pode ser escrito seguindo algumas orientações:
>
> Cada linha deste arquivo é composta de uma dupla: rótulo (decimal) e endereço (hexadecimal). Qualquer outra informação é vista como um comentário. Um exemplo pode ser visto na Figura 11. Os rótulos possíveis são:
>
> - Rótulo “0”: leitura de dados;
> - Rótulo “1”: gravação de dados;
> - Rótulo “2”: busca de instrução;
> - Rótulo “3”: registro escape (tratado como tipo de acesso desconhecido);
> - Rótulo “4”: registro escape (operação de cache flush).
> Amnesia, Tutorial técnico do módulo memória virtual. Arquivos de trace.

Somente os rótulos de leitura e gravação de dados (0 e 1 respectivamente) serão utilizados, isso porque será avaliado o impacto do número de cache hits que independe de operações que não sejam essas duas informadas. E assim, tendo em vista tudo o que foi explicado, a proposta é criar código capaz de executar o algoritmo quicksort em três cenários (melhor caso, caso médio e pior caso) e imprimir seus respectivos traces compatíveis com a execução e o simulador Amnesia.

### Endereços

Para simular a real alocação dos endereços no trace é desenvolvido duas variáveis globais de controle: `ADRS_HASH` e `PC`. `ADRS_HASH` é uma tabela hash onde as chaves são os endereços reais das variáveis usadas no código e os valores endereços falsos e menores para representarem suas chaves no trace. `PC` é um número natural de oito bits usado para gerar os valores de `ADRS_HASH`, onde para cada nova chave seu valor é igual ao conteúdo de `PC` e após isso é adicionado 1 ao `PC`, prevenindo assim dados repetidos.

```rust
let pc = state::get(&PC);
let tmp = pc + 1;
adrs_hash.insert(value, tmp);
state::set(&ADRS_HASH, adrs_hash);
state::set(&PC, tmp);
new_value = tmp;
```
[https://github.com/felipe-tertuliano/amnesia_quicksort/blob/main/src/tracer/mod.rs](https://github.com/felipe-tertuliano/amnesia_quicksort/blob/main/src/tracer/mod.rs). Linha 27 até 35.

### Geração do trace

A geração do trace e dada pela função `rw` que recebe como parâmetro dois vetores de endereços: um de leitura e outro de escrita, assim se consegue replicar a execução de linhas de código como traces. Junto disso a função `adrs` é usada para converter o ponteiro da variável em um valor legível para `ADRS_HASH`, segue uma tabela com os exemplos de uso das funções:

| Linha de código | Equivalente com `rw` + `adrs` |
| - | - |
| `if a == true {` | `rw(&[adrs(&a)], &[]);` |
| `a = 2` | `rw(&[], &[adrs(&a)]);` |
| `a = b + c;` | `rw(&[adrs(&b), adrs(&c)], &[adrs(&a)]);` |

```rust
pub fn adrs<T>(value: &T) -> usize {
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
[https://github.com/felipe-tertuliano/amnesia_quicksort/blob/main/src/tracer/mod.rs](https://github.com/felipe-tertuliano/amnesia_quicksort/blob/main/src/tracer/mod.rs). Linha 41 até 52.

### Implementação do quicksort

O Algoritmo quicksort foi implementado na função `trace_quicksort`, onde para cada linha de código que lê ou escreve em uma variável existe uma linha adjacente que faz sua equivalência gerando o trace. Para a seleção do pivô se seleciona o valor que se encontra no meio do conjunto a ser ordenado e para a troca de valores dentro do vetor é usada a função `trace_swap` também mapeada com geração de trace.

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
			rw(&[adrs(&i), adrs(&arr), adrs(&pivot)], &[]);

			while arr[j as usize] > pivot {
				rw(&[adrs(&j), adrs(&arr), adrs(&pivot)], &[]);
				j = j - 1;
				rw(&[adrs(&j)], &[adrs(&j)]);
			}
			rw(&[adrs(&j), adrs(&arr), adrs(&pivot)], &[]);

			rw(&[adrs(&i), adrs(&j)], &[]);
			if i <= j {
				trace_swap(arr, &i, &j);
				i = i + 1;
				rw(&[adrs(&i)], &[adrs(&i)]);
				j = j - 1;
				rw(&[adrs(&j)], &[adrs(&j)]);
			}
		}
		rw(&[adrs(&i), adrs(&j)], &[]);

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
[https://github.com/felipe-tertuliano/amnesia_quicksort/blob/main/src/tracer/quicksort.rs](https://github.com/felipe-tertuliano/amnesia_quicksort/blob/main/src/tracer/quicksort.rs). Linha 3 até 70.

### Traces gerados

Foram gerados três cenários (melhor caso, caso médio e pior caso), cada um deles com um vetor simulando tal ambiente, que são processados pela função `trace_quicksort` e seu trace escrito no terminal. Para os testes os traces gerados foram salvos separadamente em arquivos de texto.

| Caso | Vetor |
| - | - |
| Melhor | `[1, 2, 3, 4, 5, 6]` |
| Médio | `[6, 2, 4, 3, 5, 1]` |
| Pior | `[6, 5, 4, 3, 2, 1]` |

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
[https://github.com/felipe-tertuliano/amnesia_quicksort/blob/main/src/main.rs](https://github.com/felipe-tertuliano/amnesia_quicksort/blob/main/src/main.rs). Linha 5 até 33.