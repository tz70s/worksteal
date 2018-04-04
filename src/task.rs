// Work stealing implementation in rust.
// Copyright 2018 Tzu-Chiao Yeh, Project is under MIT license.

//! This task module contains of task abstraction.

/// TaskTrait for boxing in task queue.
pub trait TaskTrait {
    type In;
    type Out;
    fn call(&self, Self::In) -> Self::Out;
}

/// Task wrapped a boxed closure as abstraction.
pub struct Task<In, Out> {
    task: Box<Fn(In) -> Out>,
}

impl<In, Out> Task<In, Out> {
    /// Create a new task.
    /// Which eat a Fn closure with single arguement and output.
    pub fn new<F>(task_fn: F) -> Self
    where
        F: Fn(In) -> Out + 'static,
    {
        Task {
            task: Box::new(task_fn),
        }
    }
}

impl<In, Out> TaskTrait for Task<In, Out> {
    type In = In;
    type Out = Out;
    fn call(&self, arg: In) -> Out {
        (self.task)(arg)
    }
}

#[cfg(test)]
mod test_task {

    use super::*;

    #[test]
    fn test_task_call() {
        let task = Task::new(|num: i32| num + 2);
        assert_eq!(4, task.call(2));
    }
}
