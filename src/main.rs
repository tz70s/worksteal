// Work stealing implementation in rust.
// Copyright 2018 Tzu-Chiao Yeh, Project is under MIT license.

mod task;
mod work_pool;
mod worker;

use std::sync::Arc;
use std::thread::{spawn, JoinHandle};
use work_pool::WorkPool;
use worker::Worker;

fn count_work(input: i32) -> i32 {
    let mut sum = 0;
    let iter_count = 10000 * input;
    for _ in 0..iter_count {
        sum += input;
    }
    sum
}

fn dispatch_works() {
    let work_pool_1 = WorkPool::new();
    let work_pool_2 = WorkPool::new();

    for _ in 0..1000 {
        // Interior mutability, no need to be the mutable work pool.
        work_pool_1.push(count_work);
        work_pool_2.push(count_work);
    }

    let arc_work_pool_1 = Arc::new(work_pool_1);
    let arc_work_pool_2 = Arc::new(work_pool_2);

    let worker_1 = Worker::new(arc_work_pool_1.clone(), vec![arc_work_pool_2.clone()]);
    let worker_2 = Worker::new(arc_work_pool_2.clone(), vec![arc_work_pool_1.clone()]);

    let mut join_handles = Vec::new();
    join_handles.push(spawn(move || {
        worker_1.poll(5);
    }));

    join_handles.push(spawn(move || {
        worker_2.poll(10);
    }));

    for handle in join_handles {
        handle.join();
    }
}

fn main() {
    println!("Basic work stealing feat random dispatch to thread pools.");
    dispatch_works();
}
