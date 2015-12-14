use EndScheduleTrait;
use SchedulerTrait;
use TaskBox;

pub struct EndScheduleOneTaskBoxNoContinuations<'a,
                                                TScheduler>
    where TScheduler: 'a +
                      SchedulerTrait {
    scheduler: &'a TScheduler,
    task_box: TaskBox<TScheduler::TTaskBoxParam>,
}

impl <'a,
      TScheduler> EndScheduleOneTaskBoxNoContinuations<'a,
                                                       TScheduler>
    where TScheduler: SchedulerTrait {
    pub fn new(scheduler: &'a TScheduler,
               task_box: TaskBox<TScheduler::TTaskBoxParam>) -> EndScheduleOneTaskBoxNoContinuations<'a,
                                                                                                     TScheduler> {
        EndScheduleOneTaskBoxNoContinuations { scheduler: scheduler,
                                               task_box: task_box }
    }
}

impl <'a,
      TScheduler> EndScheduleTrait for EndScheduleOneTaskBoxNoContinuations<'a,
                                                                            TScheduler>
    where TScheduler: SchedulerTrait {
    type TEndScheduleReturn = TScheduler::TScheduleReturn;

    fn end_schedule(self) -> Self::TEndScheduleReturn {
        let task_box = self.task_box;

        self.scheduler.schedule(task_box)
    }
}
