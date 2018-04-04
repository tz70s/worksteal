// Work stealing implementation in rust.
// Copyright 2018 Tzu-Chiao Yeh, Project is under MIT license.

mod task;

use task::Task;
use task::TaskTrait;

struct Node<In, Out> {
    task: Box<TaskTrait<In = In, Out = Out>>,
    next: Option<Box<Node<In, Out>>>,
}

/*
FIXME: Investigate this nested relation ships.
impl<In, Out> Node<In, Out> {
    pub fn new<F>(task_fn: F) -> Self
    where
        F: Fn(In) -> Out + 'static,
    {
        let _task = Task::new(task_fn);
        Node {
            task: Box::new(_task),
            next: None
        }
    }
}
*/

/// WorkPool structure for each workers.
struct WorkPool<In, Out> {
    head: Option<Box<Node<In, Out>>>,
    len: usize,
}

impl<In, Out> WorkPool<In, Out> {
    fn new() -> Self {
        WorkPool { head: None, len: 0 }
    }

    fn push() {
        unimplemented!();
    }

    fn pop() {
        unimplemented!();
    }

    fn steal() {
        unimplemented!();
    }
}

fn main() {
    println!("Basic work stealing feat random dispatch to thread pools.");
}
