# Medians_and_Orders

Algoritmos para extrair mínimo, máximo, mediana e n-ésimo elemento de vetor de números, escritos em Rust

Extrair o mínimo e o máximo não tem segredo. Mas extrair o n-ésimo elemento escrevendo um algoritmo

de ordem n é não trivial. Pois ordenar com um heap-sort e então pegar o n-ésimo é simples, mas é um algoritmo

de orden n * log(n).

Enquanto uma ordenação com o heap sort, 100 experimentos para listas de 5000 números leva ~22ms, 

para extrair o n-ésimo elemento levamos apenas ~4.5ms

Function: Mínimo
Número de experimentos: 100
Tamanho da lista de números: 5000
Tempo total: 2.267µs

Function: Máximo
Número de experimentos: 100
Tamanho da lista de números: 5000
Tempo total: 2.247µs

Function: Mínimo-Máximo
Número de experimentos: 100
Tamanho da lista de números: 5000
Tempo total: 2.222µs

Function: Select n-ésimo
Número de experimentos: 100
Tamanho da lista de números: 5000
Tempo total: 4.215758ms



