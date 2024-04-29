# CountingSort

Algoritmos de ordenação de vetor de números escritos em Rust

O Counting Sort é imbatível quando se trata de um vetor com elementos contidos num Range {start: 0, end: k},

onde k não é grande. 

Em todos os nossos exemplos usamos 100 experimentos com vetores de 5000 elementos. Se limitarmos k em no máximo 150000,

o Counting Sort bate o Heap Sort. Acima desse valor de k é melhor usar o Heap Sort (o mais rápido de todos sem restrições).

cargo run --release 100 5000 150000

Function: Counting_sort
Número de experimentos: 100
Tamanho da lista de números: 5000
Valor de k: 150000
Tempo total: 22.824805ms

O detalhe é que o Counting Sort é estável, enquanto o Heap Sort não é. A versão estável do Heap Sort é mais lenta.

Nesse caso, podemos aumentar k até 250000 para empatar com o Heap Sort estável.

