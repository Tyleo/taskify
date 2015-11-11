#![feature(alloc)]
#![feature(coerce_unsized)]
#![feature(fnbox)]
#![feature(heap_api)]
#![feature(shared)]
#![feature(unsize)]

extern crate alloc;
extern crate deque;
extern crate rand;

mod decay_ptr;
pub use decay_ptr::DecayPtr;

mod task_scheduler;
pub use task_scheduler::TaskScheduler;

// mod task_scheduler_builder;
// pub use task_scheduler_builder::TaskSchedulerBuilder;

mod task;
pub use task::Task;

mod task_box;
pub use task_box::TaskBox;

mod task_box_into_iterator;
pub use task_box_into_iterator::TaskBoxIntoIterator;

// Begin test area

use rand::Rng;
use rand::StdRng;

use std::thread;
use std::thread::JoinHandle;

#[test]
fn it_works() {
    // let num_threads = 8;
    // let mut rngs = Vec::<Box<Rng>>::new();
    // rngs.reserve(num_threads);

    // for i in 0..num_threads {
    //     match StdRng::new() {
    //         Ok(rng) => rngs.push(Box::new(rng)),
    //         Err(_) => panic!(),
    //     }
    // }

    // let task_scheduler_builder = match TaskSchedulerBuilder::new(num_threads, rngs) {
    //     Some(result) => result,
    //     None => panic!(),
    // };

    // let mut schedulers = task_scheduler_builder.get_task_schedulers();
    // let mut join_handles = Vec::<JoinHandle<()>>::new();
    // join_handles.reserve(num_threads);

    // for scheduler in &schedulers {
    //     for i in 0..100000 {
    //         // scheduler.add_task_0(move |task_scheduler: &TaskScheduler| {

    //         // });
    //     }
    // }

    // for mut scheduler in schedulers {
    //     let join_handle = thread::spawn(move || {
    //         scheduler.run();
    //     });

    //     join_handles.push(join_handle);
    // }

    // for join_handle in join_handles {
    //     join_handle.join();
    // }
}