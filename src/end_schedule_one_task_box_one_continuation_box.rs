use Scheduler;
use EndScheduleTrait;
use TaskBox;

pub struct EndScheduleOneTaskBoxOneContinuationBox<'a> {
    scheduler: &'a Scheduler,
    task_box: TaskBox,
    continuation_box: TaskBox,
}

impl <'a> EndScheduleOneTaskBoxOneContinuationBox<'a> {
    pub fn new(scheduler: &'a Scheduler,
               task_box: TaskBox,
               continuation_box: TaskBox) -> EndScheduleOneTaskBoxOneContinuationBox<'a> {
        EndScheduleOneTaskBoxOneContinuationBox { scheduler: scheduler,
                                                  task_box: task_box,
                                                  continuation_box: continuation_box }
    }
}

impl <'a> EndScheduleTrait<()> for EndScheduleOneTaskBoxOneContinuationBox<'a> {
    fn end_schedule(self) {
        
    }
}
