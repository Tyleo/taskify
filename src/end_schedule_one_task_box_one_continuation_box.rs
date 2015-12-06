use EndScheduleTrait;
use SchedulerTrait;
use TaskBox;

pub struct EndScheduleOneTaskBoxOneContinuationBox<'a,
                                                   TScheduler>
    where TScheduler: 'a +
                      SchedulerTrait {
    scheduler: &'a TScheduler,
    task_box: TaskBox,
    continuation_box: TaskBox,
}

impl <'a,
      TScheduler> EndScheduleOneTaskBoxOneContinuationBox<'a,
                                                          TScheduler>
    where TScheduler: SchedulerTrait {
    pub fn new(scheduler: &'a TScheduler,
               task_box: TaskBox,
               continuation_box: TaskBox) -> EndScheduleOneTaskBoxOneContinuationBox<'a,
                                                                                     TScheduler> {
        EndScheduleOneTaskBoxOneContinuationBox { scheduler: scheduler,
                                                  task_box: task_box,
                                                  continuation_box: continuation_box }
    }
}

impl <'a,
      TScheduler> EndScheduleTrait for EndScheduleOneTaskBoxOneContinuationBox<'a,
                                                                               TScheduler>
    where TScheduler: SchedulerTrait {
    type TEndScheduleReturn = TScheduler::TScheduleReturn;
    
    fn end_schedule(self) -> Self::TEndScheduleReturn {
        self.scheduler.schedule()
    }
}
