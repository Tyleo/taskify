use Scheduler;
use ScheduleTrait;

pub struct ScheduleOneTaskOneContinuation<'a> {
    scheduler: &'a Scheduler,
}

impl <'a> ScheduleOneTaskOneContinuation<'a> {
    pub fn new(scheduler: &'a Scheduler) -> ScheduleOneTaskOneContinuation<'a> {
        ScheduleOneTaskOneContinuation { scheduler: scheduler }
    }
}

impl <'a> ScheduleTrait for ScheduleOneTaskOneContinuation<'a> {
    fn schedule(self) {
        
    }
}
