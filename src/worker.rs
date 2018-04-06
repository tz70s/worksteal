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
    /// Poll is represented as the mixing operations from pop and steal.
    pub fn poll(&self, arg: In) {
        loop {
            // Check if there's any workloads in local queue.
            let task = self.local_pool.pop();
            // If existed, call it, else, continue to steal.
            task.map(|t| t.call(arg.clone()));

            // Else, randomly pick up next task.
            for victim in self.others.iter() {
                let task = victim.pop();
                task.map(|t| t.call(arg.clone()));
            }
        }
    }
}
