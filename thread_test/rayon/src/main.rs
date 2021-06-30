use rand::prelude::*;
use rayon::prelude::*;
use std::time::Instant;

fn fib(i: u64) -> u64 {
    match i {
        0 => 1,
        1 => 1,
        _ => fib(i-2) + fib(i-1),
    }
}

fn main() {
    let mut rng = thread_rng();
    let xs: Vec<_> = (0..100000).map(|_| rng.gen_range(0..24)).collect();
    println!("vec length is {}", xs[0]);
    let ys = xs.clone();

    let begin = Instant::now();
    let max = xs.par_iter().map(|&x| fib(x)).max(); // `par_iter`を使って並列化
    println!("Parallel: {:?}; elapsed: {:?}", max, begin.elapsed());

    let begin = Instant::now();
    let max = ys.iter().map(|&x| fib(x)).max(); // 普通のIterator
    println!("Sync: {:?}; elapsed: {:?}", max, begin.elapsed());
}