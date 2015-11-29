use Scheduler;
use ScheduleTrait;
use TaskBox;

pub struct ScheduleOneTaskMultipleContinuations<'a> {
    scheduler: &'a Scheduler,
    task_box: TaskBox,
    continuation_boxes: Vec<TaskBox>,
}

impl <'a> ScheduleOneTaskMultipleContinuations<'a> {
    pub fn new(scheduler: &'a Scheduler,
               task_box: TaskBox,
               continuation_boxes: Vec<TaskBox>) -> ScheduleOneTaskMultipleContinuations<'a> {
        ScheduleOneTaskMultipleContinuations { scheduler: scheduler,
                                               task_box: task_box,
                                               continuation_boxes: continuation_boxes }
    }
}

impl <'a> ScheduleTrait for ScheduleOneTaskMultipleContinuations<'a> {
    fn schedule(self) {
        
    }
}
