#![feature(alloc)]
#![feature(coerce_unsized)]
#![feature(fnbox)]
#![feature(heap_api)]
#![feature(shared)]
#![feature(unsize)]

extern crate alloc;
extern crate deque;
extern crate rand;

mod begin_schedule_trait;
pub use begin_schedule_trait::BeginScheduleTrait;

mod continuation_adder_multiple_task_boxes_multiple_continuation_boxes;
pub use continuation_adder_multiple_task_boxes_multiple_continuation_boxes::ContinuationAdderMultipleTaskBoxesMultipleContinuationBoxes;

mod continuation_adder_multiple_task_boxes_one_continuation_box;
pub use continuation_adder_multiple_task_boxes_one_continuation_box::ContinuationAdderMultipleTaskBoxesOneContinuationBox;

mod continuation_adder_one_task_box_multiple_continuation_boxes;
pub use continuation_adder_one_task_box_multiple_continuation_boxes::ContinuationAdderOneTaskBoxMultipleContinuationBoxes;

mod continuation_adder_one_task_box_one_continuation_box;
pub use continuation_adder_one_task_box_one_continuation_box::ContinuationAdderOneTaskBoxOneContinuationBox;

mod continuation_adder_trait;
pub use continuation_adder_trait::ContinuationAdderTrait;

mod decay_ptr;
pub use decay_ptr::DecayPtr;

mod empty_task_adder;
pub use empty_task_adder::EmptyTaskAdder;

mod empty_task_adder_trait;
pub use empty_task_adder_trait::EmptyTaskAdderTrait;

mod end_schedule_multiple_task_boxes_multiple_continuation_boxes;
pub use end_schedule_multiple_task_boxes_multiple_continuation_boxes::EndScheduleMultipleTaskBoxesMultipleContinuationBoxes;

mod end_schedule_multiple_task_boxes_no_continuations;
pub use end_schedule_multiple_task_boxes_no_continuations::EndScheduleMultipleTaskBoxesNoContinuations;

mod end_schedule_multiple_task_boxes_one_continuation_box;
pub use end_schedule_multiple_task_boxes_one_continuation_box::EndScheduleMultipleTaskBoxesOneContinuationBox;

mod end_schedule_one_task_box_multiple_continuation_boxes;
pub use end_schedule_one_task_box_multiple_continuation_boxes::EndScheduleOneTaskBoxMultipleContinuationBoxes;

mod end_schedule_one_task_box_no_continuations;
pub use end_schedule_one_task_box_no_continuations::EndScheduleOneTaskBoxNoContinuations;

mod end_schedule_one_task_box_one_continuation_box;
pub use end_schedule_one_task_box_one_continuation_box::EndScheduleOneTaskBoxOneContinuationBox;

mod end_schedule_trait;
pub use end_schedule_trait::EndScheduleTrait;

mod loose_continuation;
pub use loose_continuation::LooseContinuation;

mod scheduler;
pub use scheduler::Scheduler;

mod scheduler_trait;
pub use scheduler_trait::SchedulerTrait;

mod simple_begin_schedule_trait;
pub use simple_begin_schedule_trait::SimpleBeginScheduleTrait;

mod task;
pub use task::Task;

mod task_adder_multiple_task_boxes;
pub use task_adder_multiple_task_boxes::TaskAdderMultipleTaskBoxes;

mod task_adder_one_task_box;
pub use task_adder_one_task_box::TaskAdderOneTaskBox;

mod task_adder_trait;
pub use task_adder_trait::TaskAdderTrait;

mod task_box;
pub use task_box::TaskBox;

mod task_box_into_iterator;
pub use task_box_into_iterator::TaskBoxIntoIterator;

// Begin test area
use rand::StdRng;
use std::sync::Arc;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;
use std::thread;

#[test]
fn it_works() {
    let num_threads = 8;
    let num_tasks_per_thread = 5000000;

    let rngs: Vec<_> = (0..num_threads).map(
        |i| {
            match StdRng::new() {
                Ok(rng) =>{
                    rng
                },
                Err(_) => {
                    panic!();
                },
            }
        }).collect();
    let schedulers = Scheduler::new_batch(rngs);

    let shared = Arc::new(AtomicUsize::new(0));
    for scheduler in &schedulers {
        for i in 0..num_tasks_per_thread {
            let clone = shared.clone();

            let loose_continuation =
                LooseContinuation::<Scheduler<_>>::new()
                    .begin_schedule()
                    .add_task(
                        move |scheduler: &Scheduler<StdRng>| {
                            clone.fetch_add(1, Ordering::Relaxed);
                            let mut value = 0;
                            for i in 0..1000 {
                                value = i + 5;
                            }
                        }
                    )
                    .end_schedule();

            scheduler.schedule(loose_continuation);
        }
    }

    let join_handles: Vec<_> =
        schedulers
            .into_iter()
            .map(
                |mut scheduler| {
                    thread::spawn(
                        move || {
                            scheduler.run();
                        }
                    )
                }
            ).collect();

    for join_handle in join_handles {
        join_handle.join();
    }

    let shared_as_usize: usize = shared.load(Ordering::Relaxed);
    println!("{0}", shared_as_usize);
}
