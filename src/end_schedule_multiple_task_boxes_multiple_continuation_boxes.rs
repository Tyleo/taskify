use EndScheduleTrait;
use SchedulerTrait;
use TaskBox;

pub struct EndScheduleMultipleTaskBoxesMultipleContinuationBoxes<'a,
                                                                 TScheduler>
    where TScheduler: 'a +
                      SchedulerTrait {
    scheduler: &'a TScheduler,
    task_boxes: Vec<TaskBox>,
    continuation_boxes: Vec<TaskBox>,
}

impl <'a,
      TScheduler> EndScheduleMultipleTaskBoxesMultipleContinuationBoxes<'a,
                                                                        TScheduler>
    where TScheduler: SchedulerTrait {
    pub fn new(scheduler: &'a TScheduler,
               task_boxes: Vec<TaskBox>,
               continuation_boxes: Vec<TaskBox>) -> EndScheduleMultipleTaskBoxesMultipleContinuationBoxes<'a,
                                                                                                          TScheduler> {
        EndScheduleMultipleTaskBoxesMultipleContinuationBoxes { scheduler: scheduler,
                                                                task_boxes: task_boxes,
                                                                continuation_boxes: continuation_boxes }
    }
}

impl <'a,
      TScheduler> EndScheduleTrait for EndScheduleMultipleTaskBoxesMultipleContinuationBoxes<'a,
                                                                                             TScheduler>
    where TScheduler: SchedulerTrait {
    type TEndScheduleReturn = TScheduler::TScheduleReturn;

    fn end_schedule(self) -> Self::TEndScheduleReturn {
        self.scheduler.schedule()
    }
}
