#![feature(alloc)]
#![feature(fnbox)]
#![feature(heap_api)]
#![feature(shared)]

extern crate alloc;
extern crate deque;
extern crate rand;

pub mod fluent;

// An error with the compiler requires all these modules be public now.

pub mod loose_continuation;

pub mod scheduler;

pub mod scheduler_trait;

pub mod task;

pub mod task_box;

pub mod task_box_into_iterator;

pub mod utilities;

pub use loose_continuation::LooseContinuation;

pub use scheduler::Scheduler;

pub use scheduler_trait::SchedulerTrait;

pub use task::Task;

pub use task_box::TaskBox;

pub use task_box_into_iterator::TaskBoxIntoIterator;

#[cfg(test)]
mod tests;
