use Scheduler;
use ScheduleTrait;
use TaskBox;

pub struct ScheduleMultipleTasksNoContinuations<'a> {
    scheduler: &'a Scheduler,
    task_boxes: Vec<TaskBox>,
}

impl <'a> ScheduleMultipleTasksNoContinuations<'a> {
    pub fn new(scheduler: &'a Scheduler,
               task_boxes: Vec<TaskBox>) -> ScheduleMultipleTasksNoContinuations<'a> {
        ScheduleMultipleTasksNoContinuations { scheduler: scheduler,
                                               task_boxes: task_boxes }
    }
}

impl <'a> ScheduleTrait for ScheduleMultipleTasksNoContinuations<'a> {
    fn schedule(self) {
        
    }
}
