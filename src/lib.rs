#![feature(alloc)]
#![feature(coerce_unsized)]
#![feature(fnbox)]
#![feature(heap_api)]
#![feature(shared)]
#![feature(unsize)]

extern crate alloc;
extern crate deque;
extern crate rand;

mod continuation_adder_multiple_tasks_multiple_continuations;
pub use continuation_adder_multiple_tasks_multiple_continuations::ContinuationAdderMultipleTasksMultipleContinuations;

mod continuation_adder_multiple_tasks_one_continuation;
pub use continuation_adder_multiple_tasks_one_continuation::ContinuationAdderMultipleTasksOneContinuation;

mod continuation_adder_one_task_multiple_continuations;
pub use continuation_adder_one_task_multiple_continuations::ContinuationAdderOneTaskMultipleContinuations;

mod continuation_adder_one_task_one_continuation;
pub use continuation_adder_one_task_one_continuation::ContinuationAdderOneTaskOneContinuation;

mod continuation_adder_trait;
pub use continuation_adder_trait::ContinuationAdderTrait;

mod decay_ptr;
pub use decay_ptr::DecayPtr;

mod schedule_multiple_tasks_multiple_continuations;
pub use schedule_multiple_tasks_multiple_continuations::ScheduleMultipleTasksMultipleContinuations;

mod schedule_multiple_tasks_no_continuations;
pub use schedule_multiple_tasks_no_continuations::ScheduleMultipleTasksNoContinuations;

mod schedule_multiple_tasks_one_continuation;
pub use schedule_multiple_tasks_one_continuation::ScheduleMultipleTasksOneContinuation;

mod schedule_one_task_multiple_continuations;
pub use schedule_one_task_multiple_continuations::ScheduleOneTaskMultipleContinuations;

mod schedule_one_task_no_continuations;
pub use schedule_one_task_no_continuations::ScheduleOneTaskNoContinuations;

mod schedule_one_task_one_continuation;
pub use schedule_one_task_one_continuation::ScheduleOneTaskOneContinuation;

mod schedule_trait;
pub use schedule_trait::ScheduleTrait;

mod loose_continuation;
pub use loose_continuation::LooseContinuation;

mod loose_continuation_into_iterator;
pub use loose_continuation_into_iterator::LooseContinuationIntoIterator;

mod scheduler;
pub use scheduler::Scheduler;

mod task;
pub use task::Task;

mod task_adder_has_no_tasks_trait;
pub use task_adder_has_no_tasks_trait::TaskAdderHasNoTasksTrait;

mod task_adder_has_tasks_trait;
pub use task_adder_has_tasks_trait::TaskAdderHasTasksTrait;

mod task_adder_multiple_tasks;
pub use task_adder_multiple_tasks::TaskAdderMultipleTasks;

mod task_adder_one_task;
pub use task_adder_one_task::TaskAdderOneTask;

mod task_box;
pub use task_box::TaskBox;

mod task_box_into_iterator;
pub use task_box_into_iterator::TaskBoxIntoIterator;

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
