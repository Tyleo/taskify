use Scheduler;
use EndScheduleTrait;
use TaskBox;

pub struct EndScheduleOneTaskBoxMultipleContinuationBoxes<'a> {
    scheduler: &'a Scheduler,
    task_box: TaskBox,
    continuation_boxes: Vec<TaskBox>,
}

impl <'a> EndScheduleOneTaskBoxMultipleContinuationBoxes<'a> {
    pub fn new(scheduler: &'a Scheduler,
               task_box: TaskBox,
               continuation_boxes: Vec<TaskBox>) -> EndScheduleOneTaskBoxMultipleContinuationBoxes<'a> {
        EndScheduleOneTaskBoxMultipleContinuationBoxes { scheduler: scheduler,
                                                  task_box: task_box,
                                                  continuation_boxes: continuation_boxes }
    }
}

impl <'a> EndScheduleTrait<()> for EndScheduleOneTaskBoxMultipleContinuationBoxes<'a> {
    fn end_schedule(self) {
        
    }
}
