use Scheduler;
use ScheduleTrait;
use TaskBox;

pub struct ScheduleMultipleTasksMultipleContinuations<'a> {
    scheduler: &'a Scheduler,
    task_boxes: Vec<TaskBox>,
    continuation_boxes: Vec<TaskBox>,
}

impl <'a> ScheduleMultipleTasksMultipleContinuations<'a> {
    pub fn new(scheduler: &'a Scheduler,
               task_boxes: Vec<TaskBox>,
               continuation_boxes: Vec<TaskBox>) -> ScheduleMultipleTasksMultipleContinuations<'a> {
        ScheduleMultipleTasksMultipleContinuations { scheduler: scheduler,
                                                     task_boxes: task_boxes,
                                                     continuation_boxes: continuation_boxes }
    }
}

impl <'a> ScheduleTrait for ScheduleMultipleTasksMultipleContinuations<'a> {
    fn schedule(self) {
        
    }
}
