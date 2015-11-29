#![feature(alloc)]
#![feature(coerce_unsized)]
#![feature(fnbox)]
#![feature(heap_api)]
#![feature(shared)]
#![feature(unsize)]

extern crate alloc;
extern crate deque;
extern crate rand;

mod continuation_adder;
pub use continuation_adder::ContinuationAdder;

mod continuation_adder_trait;
pub use continuation_adder_trait::ContinuationAdderTrait;

mod end_schedule;
pub use end_schedule::EndSchedule;

mod end_schedule_trait;
pub use end_schedule_trait::EndScheduleTrait;

mod loose_continuation;
pub use loose_continuation::LooseContinuation;

mod loose_continuation_into_iterator;
pub use loose_continuation_into_iterator::LooseContinuationIntoIterator;

mod task;
pub use task::Task;

mod task_adder;
pub use task_adder::TaskAdder;

mod task_adder_trait;
pub use task_adder_trait::TaskAdderTrait;

mod task_box;
pub use task_box::TaskBox;

mod task_box_into_iterator;
pub use task_box_into_iterator::TaskBoxIntoIterator;

mod task_scheduler_1;
pub use task_scheduler_1::TaskScheduler1;

mod task_scheduler_trait;
pub use task_scheduler_trait::TaskSchedulerTrait;






mod decay_ptr;
pub use decay_ptr::DecayPtr;

mod task_scheduler;
pub use task_scheduler::TaskScheduler;







// Begin test area

use rand::Rng;
use rand::StdRng;

use std::sync::Arc;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;

use std::thread;
use std::thread::JoinHandle;


#[test]
fn it_works2() {

}

#[test]
fn it_works() {
    // let num_threads = 8;
    // let mut rngs = Vec::<Box<Rng + Send>>::new();
    // rngs.reserve(num_threads);

    // for i in 0..num_threads {
    //     match StdRng::new() {
    //         Ok(rng) => rngs.push(Box::new(rng)),
    //         Err(_) => panic!(),
    //     }
    // }

    // let schedulers = match TaskScheduler::new_batch(num_threads, rngs) {
    //     Some(result) => result,
    //     None => panic!(),
    // };

    // let shared = Arc::new(AtomicUsize::new(0));
    // for scheduler in &schedulers {
    //     for i in 0..5000000 {
    //         let mine = shared.clone();
    //         scheduler.add_task(move |task_scheduler: &TaskScheduler| {
    //             let value = mine.fetch_add(1, Ordering::Relaxed);
    //             for i in 0..100 {
    //                 let x = i;
    //             }
    //         });
    //     }
    // }

    // let mut join_handles = Vec::<JoinHandle<()>>::new();
    // join_handles.reserve(num_threads);

    // for mut scheduler in schedulers {
    //     let join_handle = thread::spawn(move || {
    //         scheduler.run();
    //     });

    //     join_handles.push(join_handle);
    // }

    // for join_handle in join_handles {
    //     join_handle.join();
    // }

    // let shared_as_usize: usize = shared.load(Ordering::Relaxed);
    // println!("{0}", shared_as_usize);
}
