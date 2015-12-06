use Scheduler;
use EndScheduleTrait;
use TaskBox;

pub struct EndScheduleMultipleTaskBoxesNoContinuations<'a> {
    scheduler: &'a Scheduler,
    task_boxes: Vec<TaskBox>,
}

impl <'a> EndScheduleMultipleTaskBoxesNoContinuations<'a> {
    pub fn new(scheduler: &'a Scheduler,
               task_boxes: Vec<TaskBox>) -> EndScheduleMultipleTaskBoxesNoContinuations<'a> {
        EndScheduleMultipleTaskBoxesNoContinuations { scheduler: scheduler,
                                                  task_boxes: task_boxes }
    }
}

impl <'a> EndScheduleTrait<()> for EndScheduleMultipleTaskBoxesNoContinuations<'a> {
    fn end_schedule(self) {
        
    }
}
