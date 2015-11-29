use Scheduler;
use EndScheduleTrait;
use TaskBox;

pub struct EndScheduleMultipleTasksMultipleContinuations<'a> {
    scheduler: &'a Scheduler,
    task_boxes: Vec<TaskBox>,
    continuation_boxes: Vec<TaskBox>,
}

impl <'a> EndScheduleMultipleTasksMultipleContinuations<'a> {
    pub fn new(scheduler: &'a Scheduler,
               task_boxes: Vec<TaskBox>,
               continuation_boxes: Vec<TaskBox>) -> EndScheduleMultipleTasksMultipleContinuations<'a> {
        EndScheduleMultipleTasksMultipleContinuations { scheduler: scheduler,
                                                        task_boxes: task_boxes,
                                                        continuation_boxes: continuation_boxes }
    }
}

impl <'a> EndScheduleTrait for EndScheduleMultipleTasksMultipleContinuations<'a> {
    fn end_schedule(self) {
        
    }
}
