// Work stealing implementation in rust.
// Copyright 2018 Tzu-Chiao Yeh, Project is under MIT license.

//! This work_pool module contains work pool implementation.

use task::Task;
// TODO: temporary undergoing replacement.
use std::collections::VecDeque;
use std::sync::Mutex;

struct Node<In, Out> {
    task: Task<In, Out>,
    next: Option<Box<Node<In, Out>>>,
}

impl<In, Out> Node<In, Out> {
    pub fn new<F>(task_fn: F) -> Self
    where
        F: Fn(In) -> Out + 'static + Send + Sync,
    {
        let _task = Task::new(task_fn);
        Node {
            task: _task,
            next: None,
        }
    }
}

/// WorkPool structure for each workers.
/// The head, tail should be atomic based ptr.
pub struct WorkPool<In, Out> {
    pool: Mutex<VecDeque<Task<In, Out>>>,
}

impl<In, Out> WorkPool<In, Out> {
    pub fn new() -> Self {
        WorkPool {
            pool: Mutex::new(VecDeque::new()),
        }
    }

    pub fn push<F>(&self, task_fn: F)
    where
        F: Fn(In) -> Out + 'static + Send + Sync,
    {
        let _task = Task::new(task_fn);
        self.pool.lock().unwrap().push_back(_task);
    }

    pub fn pop(&self) -> Option<Task<In, Out>> {
        self.pool.lock().unwrap().pop_back()
    }

    pub fn steal(&self) -> Option<Task<In, Out>> {
        self.pool.lock().unwrap().pop_front()
    }
}
