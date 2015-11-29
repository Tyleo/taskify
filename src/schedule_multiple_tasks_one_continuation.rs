use Scheduler;
use ScheduleTrait;

pub struct ScheduleMultipleTasksOneContinuation<'a> {
    scheduler: &'a Scheduler,
}

impl <'a> ScheduleMultipleTasksOneContinuation<'a> {
    pub fn new(scheduler: &'a Scheduler) -> ScheduleMultipleTasksOneContinuation<'a> {
        ScheduleMultipleTasksOneContinuation { scheduler: scheduler }
    }
}

impl <'a> ScheduleTrait for ScheduleMultipleTasksOneContinuation<'a> {
    fn schedule(self) {
        
    }
}
