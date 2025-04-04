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

Inicialmente precisamos entender o funcionamento do simulador Amnesia e a proposta com o trace gerado no código. O simulador Amnesia tem como objetivo simular de forma visual o funcionamento da memória em um processador, para isso ele precisa de um arquivo de trace para representar um programa sendo executado, entretanto seu trace é diferente:

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
[https://github.com/felipe-tertuliano/amnesia_quicksort/blob/main/src/tracer/mod.rs](https://github.com/felipe-tertuliano/amnesia_quicksort/blob/main/src/tracer/mod.rs). linha 27 até 35.

### Gerando o trace

