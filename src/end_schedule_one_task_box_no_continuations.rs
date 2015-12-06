use Scheduler;
use EndScheduleTrait;
use TaskBox;

pub struct EndScheduleOneTaskBoxNoContinuations<'a> {
    scheduler: &'a Scheduler,
    task_box: TaskBox,
}

impl <'a> EndScheduleOneTaskBoxNoContinuations<'a> {
    pub fn new(scheduler: &'a Scheduler,
               task_box: TaskBox) -> EndScheduleOneTaskBoxNoContinuations<'a> {
        EndScheduleOneTaskBoxNoContinuations { scheduler: scheduler,
                                               task_box: task_box }
    }
}

impl <'a> EndScheduleTrait<()> for EndScheduleOneTaskBoxNoContinuations<'a> {
    fn end_schedule(self) {
        
    }
}
