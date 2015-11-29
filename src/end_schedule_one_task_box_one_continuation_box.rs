use Scheduler;
use EndScheduleTrait;
use TaskBox;

pub struct EndScheduleOneTaskOneContinuation<'a> {
    scheduler: &'a Scheduler,
    task_box: TaskBox,
    continuation_box: TaskBox,
}

impl <'a> EndScheduleOneTaskOneContinuation<'a> {
    pub fn new(scheduler: &'a Scheduler,
               task_box: TaskBox,
               continuation_box: TaskBox) -> EndScheduleOneTaskOneContinuation<'a> {
        EndScheduleOneTaskOneContinuation { scheduler: scheduler,
                                            task_box: task_box,
                                            continuation_box: continuation_box }
    }
}

impl <'a> EndScheduleTrait for EndScheduleOneTaskOneContinuation<'a> {
    fn end_schedule(self) {
        
    }
}
