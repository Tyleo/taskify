use EndScheduleTrait;
use Scheduler;
use SchedulerTrait;
use TaskBox;

pub struct EndScheduleOneTaskBoxOneContinuationBox<'a,
                                                   TScheduler>
    where TScheduler: 'a +
                      SchedulerTrait {
    scheduler: &'a TScheduler,
    task_box: TaskBox<TScheduler::TTaskBoxParam>,
    continuation_box: TaskBox<TScheduler::TTaskBoxParam>,
}

impl <'a,
      TScheduler> EndScheduleOneTaskBoxOneContinuationBox<'a,
                                                          TScheduler>
    where TScheduler: SchedulerTrait {
    pub fn new(scheduler: &'a TScheduler,
               task_box: TaskBox<TScheduler::TTaskBoxParam>,
               continuation_box: TaskBox<TScheduler::TTaskBoxParam>) -> EndScheduleOneTaskBoxOneContinuationBox<'a,
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
        let task_box = self.task_box;
        let continuation_box = self.continuation_box;

        let complete_task = move |scheduler: &TScheduler::TTaskBoxParam| {
            task_box.call_box((&scheduler,));
            continuation_box.call_box((&scheduler,));
        };
        let complete_task_box = Box::new(complete_task);

        self.scheduler.schedule(complete_task_box)
    }
}
