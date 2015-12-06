use SchedulerTrait;
use SimpleBeginScheduleTrait;

pub struct Scheduler;

impl<'a> SimpleBeginScheduleTrait<'a,
                                  Scheduler> for Scheduler { }

impl SchedulerTrait for Scheduler {
    type TScheduleReturn = ();

    fn schedule(&self) -> Self::TScheduleReturn {
        ()
    }
}
