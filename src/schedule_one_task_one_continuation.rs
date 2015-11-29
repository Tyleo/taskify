use Scheduler;
use ScheduleTrait;
use TaskBox;

pub struct ScheduleOneTaskOneContinuation<'a> {
    scheduler: &'a Scheduler,
    task_box: TaskBox,
    continuation_box: TaskBox,
}

impl <'a> ScheduleOneTaskOneContinuation<'a> {
    pub fn new(scheduler: &'a Scheduler,
               task_box: TaskBox,
               continuation_box: TaskBox) -> ScheduleOneTaskOneContinuation<'a> {
        ScheduleOneTaskOneContinuation { scheduler: scheduler,
                                         task_box: task_box,
                                         continuation_box: continuation_box }
    }
}

impl <'a> ScheduleTrait for ScheduleOneTaskOneContinuation<'a> {
    fn schedule(self) {
        
    }
}
