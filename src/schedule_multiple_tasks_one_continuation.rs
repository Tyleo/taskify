use Scheduler;
use ScheduleTrait;
use TaskBox;

pub struct ScheduleMultipleTasksOneContinuation<'a> {
    scheduler: &'a Scheduler,
    task_boxes: Vec<TaskBox>,
    continuation_box: TaskBox,
}

impl <'a> ScheduleMultipleTasksOneContinuation<'a> {
    pub fn new(scheduler: &'a Scheduler,
               task_boxes: Vec<TaskBox>,
               continuation_box: TaskBox) -> ScheduleMultipleTasksOneContinuation<'a> {
        ScheduleMultipleTasksOneContinuation { scheduler: scheduler,
                                               task_boxes: task_boxes,
                                               continuation_box: continuation_box }
    }
}

impl <'a> ScheduleTrait for ScheduleMultipleTasksOneContinuation<'a> {
    fn schedule(self) {
        
    }
}
