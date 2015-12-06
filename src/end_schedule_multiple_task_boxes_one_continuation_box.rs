use EndScheduleTrait;
use SchedulerTrait;
use TaskBox;

pub struct EndScheduleMultipleTaskBoxesOneContinuationBox<'a,
                                                          TScheduler>
    where TScheduler: 'a +
                      SchedulerTrait {
    scheduler: &'a TScheduler,
    task_boxes: Vec<TaskBox>,
    continuation_box: TaskBox,
}

impl <'a,
      TScheduler> EndScheduleMultipleTaskBoxesOneContinuationBox<'a,
                                                                 TScheduler>
    where TScheduler: SchedulerTrait {
    pub fn new(scheduler: &'a TScheduler,
               task_boxes: Vec<TaskBox>,
               continuation_box: TaskBox) -> EndScheduleMultipleTaskBoxesOneContinuationBox<'a,
                                                                                            TScheduler> {
        EndScheduleMultipleTaskBoxesOneContinuationBox { scheduler: scheduler,
                                                         task_boxes: task_boxes,
                                                         continuation_box: continuation_box }
    }
}

impl <'a,
      TScheduler> EndScheduleTrait for EndScheduleMultipleTaskBoxesOneContinuationBox<'a,
                                                                                      TScheduler>
    where TScheduler: SchedulerTrait {
    type TEndScheduleReturn = TScheduler::TScheduleReturn;

    fn end_schedule(self) -> Self::TEndScheduleReturn {
        self.scheduler.schedule()
    }
}
