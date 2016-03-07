#![feature(alloc)]
#![feature(fnbox)]
#![feature(heap_api)]
#![feature(shared)]

extern crate alloc;
extern crate deque;
extern crate rand;

pub mod fluent;

mod loose_continuation;

mod scheduler;

mod scheduler_trait;

mod task;

mod task_box;

mod task_box_into_iterator;

mod utilities;

pub use loose_continuation::LooseContinuation;

pub use scheduler::Scheduler;

pub use scheduler_trait::SchedulerTrait;

pub use task::Task;

pub use task_box::TaskBox;

pub use task_box_into_iterator::TaskBoxIntoIterator;


#[cfg(test)]
mod tests;
