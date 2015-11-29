use Scheduler;
use ScheduleTrait;

pub struct ScheduleMultipleTasksMultipleContinuations<'a> {
    scheduler: &'a Scheduler,
}

impl <'a> ScheduleMultipleTasksMultipleContinuations<'a> {
    pub fn new(scheduler: &'a Scheduler) -> ScheduleMultipleTasksMultipleContinuations<'a> {
        ScheduleMultipleTasksMultipleContinuations { scheduler: scheduler }
    }
}

impl <'a> ScheduleTrait for ScheduleMultipleTasksMultipleContinuations<'a> {
    fn schedule(self) {
        
    }
}
