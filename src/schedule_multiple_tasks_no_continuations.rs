use Scheduler;
use ScheduleTrait;

pub struct ScheduleMultipleTasksNoContinuations<'a> {
    scheduler: &'a Scheduler,
}

impl <'a> ScheduleMultipleTasksNoContinuations<'a> {
    pub fn new(scheduler: &'a Scheduler) -> ScheduleMultipleTasksNoContinuations<'a> {
        ScheduleMultipleTasksNoContinuations { scheduler: scheduler }
    }
}

impl <'a> ScheduleTrait for ScheduleMultipleTasksNoContinuations<'a> {
    fn schedule(self) {
        
    }
}
