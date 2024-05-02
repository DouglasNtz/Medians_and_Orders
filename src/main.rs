use Medians_and_Orders::{return_min, return_max, return_min_max, return_min_max_fast, randomized_select, ith_select};
use std::time::{Duration, Instant};
use rand;

fn main() {

    let args: Vec<String> = std::env::args().collect();

    let numero_experimentos = args[1].parse::<usize>().unwrap();

    let tamanho_lista = args[2].parse::<usize>().unwrap();

    let mut times = Vec::with_capacity(numero_experimentos);

    let mut start_time;
    let mut duration;

    for _exp in 0..numero_experimentos {

        let mut v = Vec::with_capacity(tamanho_lista);
        for _i in 0..tamanho_lista {
            v.push(rand::random::<i32>());
        }

        start_time = Instant::now();

        return_min(&v);

        duration = start_time.elapsed();

        times.push(duration);
    }

    println!(r###"Function: Mínimo
Número de experimentos: {numero_experimentos}
Tamanho da lista de números: {tamanho_lista}
Tempo total: {:?}
"###, times.iter().sum::<Duration>());

    //------------------

    let mut times = Vec::with_capacity(numero_experimentos);

    let mut start_time;
    let mut duration;

    for _exp in 0..numero_experimentos {

        let mut v = Vec::with_capacity(tamanho_lista);
        for _i in 0..tamanho_lista {
            v.push(rand::random::<i32>());
        }

        start_time = Instant::now();

        return_max(&v);

        duration = start_time.elapsed();

        times.push(duration);
    }

    println!(r###"Function: Máximo
Número de experimentos: {numero_experimentos}
Tamanho da lista de números: {tamanho_lista}
Tempo total: {:?}
"###, times.iter().sum::<Duration>());


    //---------------

    let args: Vec<String> = std::env::args().collect();

    let numero_experimentos = args[1].parse::<usize>().unwrap();

    let tamanho_lista = args[2].parse::<usize>().unwrap();

    let mut times = Vec::with_capacity(numero_experimentos);

    let mut start_time;
    let mut duration;

    for _exp in 0..numero_experimentos {

        let mut v = Vec::with_capacity(tamanho_lista);
        for _i in 0..tamanho_lista {
            v.push(rand::random::<i32>());
        }

        start_time = Instant::now();

        return_min_max(&v);

        duration = start_time.elapsed();

        times.push(duration);
    }

    println!(r###"Function: Mínimo-Máximo
Número de experimentos: {numero_experimentos}
Tamanho da lista de números: {tamanho_lista}
Tempo total: {:?}
"###, times.iter().sum::<Duration>());

    //------------------

    let mut times = Vec::with_capacity(numero_experimentos);

    let mut start_time;
    let mut duration;

    for _exp in 0..numero_experimentos {

        let mut v = Vec::with_capacity(tamanho_lista);
        for _i in 0..tamanho_lista {
            v.push(rand::random::<i32>());
        }

        let position = 1 + (rand::random::<usize>() % tamanho_lista);

        start_time = Instant::now();

        randomized_select(&v, position);

        duration = start_time.elapsed();

        times.push(duration);
    }

    println!(r###"Function: Rand-select n-ésimo
Número de experimentos: {numero_experimentos}
Tamanho da lista de números: {tamanho_lista}
Tempo total: {:?}
"###, times.iter().sum::<Duration>());

    //------------------

    let mut times = Vec::with_capacity(numero_experimentos);

    let mut start_time;
    let mut duration;

    for _exp in 0..numero_experimentos {

        let mut v = Vec::with_capacity(tamanho_lista);
        for _i in 0..tamanho_lista {
            v.push(rand::random::<i32>());
        }

        let position = 1 + (rand::random::<usize>() % tamanho_lista);

        start_time = Instant::now();

        ith_select(&v, position);

        duration = start_time.elapsed();

        times.push(duration);
    }

    println!(r###"Function: Select n-ésimo
Número de experimentos: {numero_experimentos}
Tamanho da lista de números: {tamanho_lista}
Tempo total: {:?}
"###, times.iter().sum::<Duration>());


}
