#![feature(alloc)]
#![feature(fnbox)]
#![feature(heap_api)]
#![feature(shared)]

extern crate alloc;
extern crate deque;
extern crate rand;


mod begin_schedule_trait;

mod continuation_adder_multiple_task_boxes_multiple_continuation_boxes;

mod continuation_adder_multiple_task_boxes_one_continuation_box;

mod continuation_adder_one_task_box_multiple_continuation_boxes;

mod continuation_adder_one_task_box_one_continuation_box;

mod continuation_adder_trait;

mod empty_task_adder;

mod empty_task_adder_trait;

mod end_schedule_multiple_task_boxes_multiple_continuation_boxes;

mod end_schedule_multiple_task_boxes_no_continuations;

mod end_schedule_multiple_task_boxes_one_continuation_box;

mod end_schedule_one_task_box_multiple_continuation_boxes;

mod end_schedule_one_task_box_no_continuations;

mod end_schedule_one_task_box_one_continuation_box;

mod end_schedule_trait;

mod loose_continuation;

mod scheduler;

mod scheduler_trait;

mod simple_begin_schedule_trait;

mod task;

mod task_adder_trait;

mod task_adder_one_task_box;

mod task_adder_multiple_task_boxes;

mod task_box;

mod task_box_into_iterator;

mod utilities;


pub use loose_continuation::LooseContinuation;

pub use scheduler::Scheduler;

pub use scheduler_trait::SchedulerTrait;

pub use task::Task;

pub use task_box::TaskBox;

pub use task_box_into_iterator::TaskBoxIntoIterator;

pub mod fluent {
    pub use begin_schedule_trait::BeginScheduleTrait;

    pub use continuation_adder_multiple_task_boxes_multiple_continuation_boxes::ContinuationAdderMultipleTaskBoxesMultipleContinuationBoxes;

    pub use continuation_adder_multiple_task_boxes_one_continuation_box::ContinuationAdderMultipleTaskBoxesOneContinuationBox;

    pub use continuation_adder_one_task_box_multiple_continuation_boxes::ContinuationAdderOneTaskBoxMultipleContinuationBoxes;

    pub use continuation_adder_one_task_box_one_continuation_box::ContinuationAdderOneTaskBoxOneContinuationBox;

    pub use continuation_adder_trait::ContinuationAdderTrait;

    pub use empty_task_adder::EmptyTaskAdder;

    pub use empty_task_adder_trait::EmptyTaskAdderTrait;

    pub use end_schedule_multiple_task_boxes_multiple_continuation_boxes::EndScheduleMultipleTaskBoxesMultipleContinuationBoxes;

    pub use end_schedule_multiple_task_boxes_no_continuations::EndScheduleMultipleTaskBoxesNoContinuations;

    pub use end_schedule_multiple_task_boxes_one_continuation_box::EndScheduleMultipleTaskBoxesOneContinuationBox;

    pub use end_schedule_one_task_box_multiple_continuation_boxes::EndScheduleOneTaskBoxMultipleContinuationBoxes;

    pub use end_schedule_one_task_box_no_continuations::EndScheduleOneTaskBoxNoContinuations;

    pub use end_schedule_one_task_box_one_continuation_box::EndScheduleOneTaskBoxOneContinuationBox;

    pub use end_schedule_trait::EndScheduleTrait;

    pub use simple_begin_schedule_trait::SimpleBeginScheduleTrait;

    pub use task_adder_trait::TaskAdderTrait;

    pub use task_adder_multiple_task_boxes::TaskAdderMultipleTaskBoxes;

    pub use task_adder_one_task_box::TaskAdderOneTaskBox;
}


#[cfg(test)]
mod tests;
