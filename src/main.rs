#![feature(thread_local)]

#[macro_use]
extern crate lazy_static;

use std::collections::HashMap;

use thread_local::ThreadLocal;
use std::sync::Arc;
use std::cell::Cell;

use rayon::{ThreadPoolBuilder, prelude::*};


fn main() {


    sum_of_squares();

}


fn sum_of_squares(){

    let range = 0..10;

    let tp = ThreadPoolBuilder::new().num_threads(4).build().unwrap();

    let mut tls = Arc::new(ThreadLocal::new());

    tp.install(|| {
        let partial_sums: Vec<usize> = range.into_par_iter().fold( || 0, |acc, next| {
            let cell = tls.get_or(|| Cell::new(0));
            cell.set(cell.get() + next*next);
            acc + next*next
        }).collect();

        println!("{:?}", partial_sums);
        println!("total inside threadpool: {}", partial_sums.iter().sum::<usize>());
    });

    let tls = Arc::try_unwrap(tls).unwrap();
    let total = tls.into_iter().fold(0, |acc, next| acc + next.get());

    println!("total from thread local: {}",total);
}