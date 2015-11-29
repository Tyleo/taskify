use Scheduler;
use EndScheduleTrait;
use TaskBox;

pub struct EndScheduleOneTaskNoContinuations<'a> {
    scheduler: &'a Scheduler,
    task_box: TaskBox,
}

impl <'a> EndScheduleOneTaskNoContinuations<'a> {
    pub fn new(scheduler: &'a Scheduler,
               task_box: TaskBox) -> EndScheduleOneTaskNoContinuations<'a> {
        EndScheduleOneTaskNoContinuations { scheduler: scheduler,
                                            task_box: task_box }
    }
}

impl <'a> EndScheduleTrait for EndScheduleOneTaskNoContinuations<'a> {
    fn end_schedule(self) {
        
    }
}
