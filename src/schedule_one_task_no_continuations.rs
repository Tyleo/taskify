use Scheduler;
use ScheduleTrait;
use TaskBox;

pub struct ScheduleOneTaskNoContinuations<'a> {
    scheduler: &'a Scheduler,
    task_box: TaskBox,
}

impl <'a> ScheduleOneTaskNoContinuations<'a> {
    pub fn new(scheduler: &'a Scheduler,
               task_box: TaskBox) -> ScheduleOneTaskNoContinuations<'a> {
        ScheduleOneTaskNoContinuations { scheduler: scheduler,
                                         task_box: task_box }
    }
}

impl <'a> ScheduleTrait for ScheduleOneTaskNoContinuations<'a> {
    fn schedule(self) {
        
    }
}
