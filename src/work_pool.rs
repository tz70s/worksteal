// Work stealing implementation in rust.
// Copyright 2018 Tzu-Chiao Yeh, Project is under MIT license.

//! This work_pool module contains work pool implementation.

use task::Task;

struct Node<In, Out> {
    task: Task<In, Out>,
    next: Option<Box<Node<In, Out>>>,
}

impl<In, Out> Node<In, Out> {
    pub fn new<F>(task_fn: F) -> Self
    where
        F: Fn(In) -> Out + 'static,
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
    head: Option<Box<Node<In, Out>>>,
    tail: Option<Box<Node<In, Out>>>,
    len: usize,
}

impl<In, Out> WorkPool<In, Out> {
    pub fn new() -> Self {
        WorkPool {
            head: None,
            tail: None,
            len: 0,
        }
    }

    pub fn push<F>(&mut self, task_fn: F)
    where
        F: Fn(In) -> Out + 'static,
    {
        match self.head {
            Some(ref mut boxed_node) => boxed_node.next = Some(Box::new(Node::new(task_fn))),
            None => self.head = Some(Box::new(Node::new(task_fn))),
        }
        self.len += 1;
    }

    pub fn pop(&self) -> Option<Task<In, Out>> {
        unimplemented!();
    }

    pub fn steal(&self) -> Option<Task<In, Out>> {
        unimplemented!();
    }
}
