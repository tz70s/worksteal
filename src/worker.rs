// Work stealing implementation in rust.
// Copyright 2018 Tzu-Chiao Yeh, Project is under MIT license.

//! This worker module contains worker implementation.

use std::sync::Arc;
use work_pool::WorkPool;

pub struct Worker<In, Out> {
    local_pool: Arc<WorkPool<In, Out>>,
    others: Vec<Arc<WorkPool<In, Out>>>,
}

impl<In, Out> Worker<In, Out>
where
    In: Clone,
{
    pub fn new(local_pool: Arc<WorkPool<In, Out>>, others: Vec<Arc<WorkPool<In, Out>>>) -> Self {
        Self { local_pool, others }
    }

    /// Add neighbor worker into steal pools.
    pub fn add_neighbors(&mut self, other: Arc<WorkPool<In, Out>>) {
        self.others.push(other);
    }

    /// Poll is represented as the mixing operations from pop and steal.
    pub fn poll(&self, arg: In) {
        loop {
            // Check if there's any workloads in local queue.
            let task = self.local_pool.pop();
            // If existed, call it, else, continue to steal.
            if let Some(t) = task {
                t.call(arg.clone());
                continue;
            }

            // Else, randomly pick up next task.
            for victim in self.others.iter() {
                let task = victim.steal();
                if let Some(t) = task {
                    t.call(arg.clone());
                    println!("Steal occurred!");
                    continue;
                }
            }
            break;
        }
    }
}
