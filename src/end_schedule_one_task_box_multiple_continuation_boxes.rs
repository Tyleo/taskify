use Scheduler;
use EndScheduleTrait;
use TaskBox;

pub struct EndScheduleOneTaskMultipleContinuations<'a> {
    scheduler: &'a Scheduler,
    task_box: TaskBox,
    continuation_boxes: Vec<TaskBox>,
}

impl <'a> EndScheduleOneTaskMultipleContinuations<'a> {
    pub fn new(scheduler: &'a Scheduler,
               task_box: TaskBox,
               continuation_boxes: Vec<TaskBox>) -> EndScheduleOneTaskMultipleContinuations<'a> {
        EndScheduleOneTaskMultipleContinuations { scheduler: scheduler,
                                                  task_box: task_box,
                                                  continuation_boxes: continuation_boxes }
    }
}

impl <'a> EndScheduleTrait<()> for EndScheduleOneTaskMultipleContinuations<'a> {
    fn end_schedule(self) {
        
    }
}
