use Scheduler;
use ScheduleTrait;

pub struct ScheduleOneTaskMultipleContinuations<'a> {
    scheduler: &'a Scheduler,
}

impl <'a> ScheduleOneTaskMultipleContinuations<'a> {
    pub fn new(scheduler: &'a Scheduler) -> ScheduleOneTaskMultipleContinuations<'a> {
        ScheduleOneTaskMultipleContinuations { scheduler: scheduler }
    }
}

impl <'a> ScheduleTrait for ScheduleOneTaskMultipleContinuations<'a> {
    fn schedule(self) {
        
    }
}
