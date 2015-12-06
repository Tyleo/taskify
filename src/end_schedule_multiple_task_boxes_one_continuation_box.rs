use Scheduler;
use EndScheduleTrait;
use TaskBox;

pub struct EndScheduleMultipleTaskBoxesOneContinuationBox<'a> {
    scheduler: &'a Scheduler,
    task_boxes: Vec<TaskBox>,
    continuation_box: TaskBox,
}

impl <'a> EndScheduleMultipleTaskBoxesOneContinuationBox<'a> {
    pub fn new(scheduler: &'a Scheduler,
               task_boxes: Vec<TaskBox>,
               continuation_box: TaskBox) -> EndScheduleMultipleTaskBoxesOneContinuationBox<'a> {
        EndScheduleMultipleTaskBoxesOneContinuationBox { scheduler: scheduler,
                                                         task_boxes: task_boxes,
                                                         continuation_box: continuation_box }
    }
}

impl <'a> EndScheduleTrait<()> for EndScheduleMultipleTaskBoxesOneContinuationBox<'a> {
    fn end_schedule(self) {
        
    }
}
