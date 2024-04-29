use CountingSort::{counting_sort};
use std::time::{Duration, Instant};
use rand;

fn main() {

    let args: Vec<String> = std::env::args().collect();

    let numero_experimentos = args[1].parse::<usize>().unwrap();

    let tamanho_lista = args[2].parse::<usize>().unwrap();

    let maximo = args[3].parse::<usize>().unwrap();

    let mut times = Vec::with_capacity(numero_experimentos);

    let mut start_time;
    let mut duration;

    for _exp in 0..numero_experimentos {
        let mut v = Vec::with_capacity(tamanho_lista);
        for _i in 0..tamanho_lista {
            v.push(rand::random::<usize>() % (maximo + 1));
        }

        start_time = Instant::now();

        counting_sort(&mut v, maximo);

        duration = start_time.elapsed();

        times.push(duration);
    }

    println!(r###"Function: Counting_sort
Número de experimentos: {numero_experimentos}
Tamanho da lista de números: {tamanho_lista}
Valor de k: {maximo}
Tempo total: {:?}
"###, times.iter().sum::<Duration>());

}