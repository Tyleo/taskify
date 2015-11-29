use Scheduler;
use EndScheduleTrait;
use TaskBox;

pub struct EndScheduleMultipleTasksNoContinuations<'a> {
    scheduler: &'a Scheduler,
    task_boxes: Vec<TaskBox>,
}

impl <'a> EndScheduleMultipleTasksNoContinuations<'a> {
    pub fn new(scheduler: &'a Scheduler,
               task_boxes: Vec<TaskBox>) -> EndScheduleMultipleTasksNoContinuations<'a> {
        EndScheduleMultipleTasksNoContinuations { scheduler: scheduler,
                                                  task_boxes: task_boxes }
    }
}

impl <'a> EndScheduleTrait for EndScheduleMultipleTasksNoContinuations<'a> {
    fn end_schedule(self) {
        
    }
}
