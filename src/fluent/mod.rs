// An error with the compiler requires all these modules be public now.

pub mod begin_schedule_trait;

pub mod continuation_adder_multiple_task_boxes_multiple_continuation_boxes;

pub mod continuation_adder_multiple_task_boxes_one_continuation_box;

pub mod continuation_adder_one_task_box_multiple_continuation_boxes;

pub mod continuation_adder_one_task_box_one_continuation_box;

pub mod continuation_adder_trait;

pub mod empty_task_adder;

pub mod empty_task_adder_trait;

pub mod end_schedule_multiple_task_boxes_multiple_continuation_boxes;

pub mod end_schedule_multiple_task_boxes_no_continuations;

pub mod end_schedule_multiple_task_boxes_one_continuation_box;

pub mod end_schedule_one_task_box_multiple_continuation_boxes;

pub mod end_schedule_one_task_box_no_continuations;

pub mod end_schedule_one_task_box_one_continuation_box;

pub mod end_schedule_trait;

pub mod simple_begin_schedule_trait;

pub mod task_adder_trait;

pub mod task_adder_multiple_task_boxes;

pub mod task_adder_one_task_box;

pub use fluent::begin_schedule_trait::BeginScheduleTrait;

pub use fluent::continuation_adder_multiple_task_boxes_multiple_continuation_boxes::ContinuationAdderMultipleTaskBoxesMultipleContinuationBoxes;

pub use fluent::continuation_adder_multiple_task_boxes_one_continuation_box::ContinuationAdderMultipleTaskBoxesOneContinuationBox;

pub use fluent::continuation_adder_one_task_box_multiple_continuation_boxes::ContinuationAdderOneTaskBoxMultipleContinuationBoxes;

pub use fluent::continuation_adder_one_task_box_one_continuation_box::ContinuationAdderOneTaskBoxOneContinuationBox;

pub use fluent::continuation_adder_trait::ContinuationAdderTrait;

pub use fluent::empty_task_adder::EmptyTaskAdder;

pub use fluent::empty_task_adder_trait::EmptyTaskAdderTrait;

pub use fluent::end_schedule_multiple_task_boxes_multiple_continuation_boxes::EndScheduleMultipleTaskBoxesMultipleContinuationBoxes;

pub use fluent::end_schedule_multiple_task_boxes_no_continuations::EndScheduleMultipleTaskBoxesNoContinuations;

pub use fluent::end_schedule_multiple_task_boxes_one_continuation_box::EndScheduleMultipleTaskBoxesOneContinuationBox;

pub use fluent::end_schedule_one_task_box_multiple_continuation_boxes::EndScheduleOneTaskBoxMultipleContinuationBoxes;

pub use fluent::end_schedule_one_task_box_no_continuations::EndScheduleOneTaskBoxNoContinuations;

pub use fluent::end_schedule_one_task_box_one_continuation_box::EndScheduleOneTaskBoxOneContinuationBox;

pub use fluent::end_schedule_trait::EndScheduleTrait;

pub use fluent::simple_begin_schedule_trait::SimpleBeginScheduleTrait;

pub use fluent::task_adder_trait::TaskAdderTrait;

pub use fluent::task_adder_multiple_task_boxes::TaskAdderMultipleTaskBoxes;

pub use fluent::task_adder_one_task_box::TaskAdderOneTaskBox;
