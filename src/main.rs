// Work stealing implementation in rust.
// Copyright 2018 Tzu-Chiao Yeh, Project is under MIT license.

use std::collections::VecDeque;
use std::thread;
use std::sync::{Arc, Mutex};
use std::time::Instant;

#[derive(Debug)]
struct WorkPool<T> {
    dequeue: VecDeque<T>,
}

// FIXME: Currently, the implementation is not correct.
// Totally relies on lock instead of CAS.
impl<T> WorkPool<T> {
    fn new() -> Self {
        WorkPool {
            dequeue: VecDeque::new(),
        }
    }

    fn push(&mut self, value: T) {
        self.dequeue.push_back(value)
    }

    fn steal(&mut self) -> Option<T> {
        self.dequeue.pop_back()
    }

    fn pop(&mut self) -> Option<T> {
        self.dequeue.pop_front()
    }
}

#[derive(Debug)]
struct Worker;

impl Worker {
    /// Spawn a worker.
    fn spawn<F>(
        &self,
        pool_1: Arc<Mutex<WorkPool<F>>>,
        pool_2: Arc<Mutex<WorkPool<F>>>,
    ) -> thread::JoinHandle<()>
    where
        F: FnOnce() + Send + Sync + 'static,
    {
        thread::spawn(move || {
            loop {
                if let Ok(mut works) = pool_1.lock() {
                    // Match if current workpool has work.
                    if let Some(task) = works.pop() {
                        // Execute task and loop back.
                        task();
                        continue;
                    }
                // Else, empty works in this work pool. Go thoughs for stealing.
                } else {
                    continue;
                }

                // Steal another work pool
                if let Ok(mut works) = pool_2.lock() {
                    if let Some(task) = works.steal() {
                        // Execute task and loop over.
                        task();
                    } else {
                        // No remain works, break loop.
                        break;
                    }
                } else {
                    // Might some buggy?
                    continue;
                }
            }
        })
    }
}

/// Sum from 0 to 10000
fn sum_number() {
    let mut sum = 0;
    for num in 0..10000 {
        sum += num;
    }
}

fn main() {
    println!("Simple work stealing implementation in rust.");

    let mut work_pool1 = WorkPool::new();
    let mut work_pool2 = WorkPool::new();

    let rounds = 10000;

    for _ in 0..rounds {
        work_pool1.push(sum_number);
        work_pool2.push(sum_number);
    }

    let arc1 = Arc::new(Mutex::new(work_pool1));
    let arc2 = Arc::new(Mutex::new(work_pool2));

    let now = Instant::now();
    let join1 = Worker.spawn(arc1.clone(), arc2.clone());
    let join2 = Worker.spawn(arc2.clone(), arc1.clone());

    join1.join().unwrap();
    join2.join().unwrap();

    println!(
        "Running {} round in {} secs",
        rounds,
        now.elapsed().as_secs()
    );
}
