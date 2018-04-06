// Work stealing implementation in rust.
// Copyright 2018 Tzu-Chiao Yeh, Project is under MIT license.

mod task;
mod work_pool;
mod worker;

fn main() {
    println!("Basic work stealing feat random dispatch to thread pools.");
}
