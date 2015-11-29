use Scheduler;
use ScheduleTrait;

pub struct ScheduleOneTaskNoContinuations<'a> {
    scheduler: &'a Scheduler,
}

impl <'a> ScheduleOneTaskNoContinuations<'a> {
    pub fn new(scheduler: &'a Scheduler) -> ScheduleOneTaskNoContinuations<'a> {
        ScheduleOneTaskNoContinuations { scheduler: scheduler }
    }
}

impl <'a> ScheduleTrait for ScheduleOneTaskNoContinuations<'a> {
    fn schedule(self) {
        
    }
}
