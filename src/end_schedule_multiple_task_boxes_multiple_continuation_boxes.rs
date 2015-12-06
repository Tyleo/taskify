use Scheduler;
use EndScheduleTrait;
use TaskBox;

pub struct EndScheduleMultipleTaskBoxesMultipleContinuationBoxes<'a> {
    scheduler: &'a Scheduler,
    task_boxes: Vec<TaskBox>,
    continuation_boxes: Vec<TaskBox>,
}

impl <'a> EndScheduleMultipleTaskBoxesMultipleContinuationBoxes<'a> {
    pub fn new(scheduler: &'a Scheduler,
               task_boxes: Vec<TaskBox>,
               continuation_boxes: Vec<TaskBox>) -> EndScheduleMultipleTaskBoxesMultipleContinuationBoxes<'a> {
        EndScheduleMultipleTaskBoxesMultipleContinuationBoxes { scheduler: scheduler,
                                                                task_boxes: task_boxes,
                                                                continuation_boxes: continuation_boxes }
    }
}

impl <'a> EndScheduleTrait<()> for EndScheduleMultipleTaskBoxesMultipleContinuationBoxes<'a> {
    fn end_schedule(self) {
        
    }
}
