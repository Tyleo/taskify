use Scheduler;
use EndScheduleTrait;
use TaskBox;

pub struct EndScheduleMultipleTasksOneContinuation<'a> {
    scheduler: &'a Scheduler,
    task_boxes: Vec<TaskBox>,
    continuation_box: TaskBox,
}

impl <'a> EndScheduleMultipleTasksOneContinuation<'a> {
    pub fn new(scheduler: &'a Scheduler,
               task_boxes: Vec<TaskBox>,
               continuation_box: TaskBox) -> EndScheduleMultipleTasksOneContinuation<'a> {
        EndScheduleMultipleTasksOneContinuation { scheduler: scheduler,
                                               task_boxes: task_boxes,
                                               continuation_box: continuation_box }
    }
}

impl <'a> EndScheduleTrait for EndScheduleMultipleTasksOneContinuation<'a> {
    fn end_schedule(self) {
        
    }
}
