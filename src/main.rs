// Work stealing implementation in rust.
// Copyright 2018 Tzu-Chiao Yeh, Project is under MIT license.

use std::collections::VecDeque;
use std::thread;
use std::sync::{Arc, Mutex, atomic::{AtomicPtr, Ordering}};
use std::time::Instant;

#[derive(Debug)]
struct Node<T> {
    data: T,
    next: Option<AtomicPtr<Node<T>>>,
}

#[derive(Debug)]
struct WorkPool<T> {
    head: AtomicPtr<Option<Node<T>>>,
    tail: AtomicPtr<Option<Node<T>>>,
}

// FIXME: Currently, the implementation is not correct.
// Totally relies on lock instead of CAS.
impl<T> WorkPool<T> {
    fn new() -> Self {
        WorkPool {
            head: AtomicPtr::new(&mut None),
            tail: AtomicPtr::new(&mut None),
        }
    }

    fn push(&mut self, value: T) {
        self.tail.store(
            &mut Some(Node {
                data: value,
                next: None,
            }),
            Ordering::Relaxed,
        );
    }

    fn steal(&mut self) -> Option<T> {
        unsafe {
            match *self.head.load(Ordering::Relaxed) {
                Some(n) => Some(n.data),
                None => None,
            }
        }
    }

    // FIXME
    fn pop(&mut self) -> Option<T> {
        match self.tail.load(Ordering::Relaxed) {
            n @ &mut Some(_) => Some(n.data),
            &mut None => None,
        }
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
