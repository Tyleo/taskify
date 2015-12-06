use EndScheduleTrait;
use SchedulerTrait;
use TaskBox;

pub struct EndScheduleOneTaskBoxMultipleContinuationBoxes<'a,
                                                          TScheduler>
    where TScheduler: 'a +
                      SchedulerTrait {
    scheduler: &'a TScheduler,
    task_box: TaskBox,
    continuation_boxes: Vec<TaskBox>,
}

impl <'a,
      TScheduler> EndScheduleOneTaskBoxMultipleContinuationBoxes<'a,
                                                                 TScheduler>
    where TScheduler: SchedulerTrait {
    pub fn new(scheduler: &'a TScheduler,
               task_box: TaskBox,
               continuation_boxes: Vec<TaskBox>) -> EndScheduleOneTaskBoxMultipleContinuationBoxes<'a,
                                                                                                   TScheduler> {
        EndScheduleOneTaskBoxMultipleContinuationBoxes { scheduler: scheduler,
                                                         task_box: task_box,
                                                         continuation_boxes: continuation_boxes }
    }
}

impl <'a,
      TScheduler> EndScheduleTrait for EndScheduleOneTaskBoxMultipleContinuationBoxes<'a,
                                                                                      TScheduler>
    where TScheduler: SchedulerTrait{
    type TEndScheduleReturn = TScheduler::TScheduleReturn;

    fn end_schedule(self) -> Self::TEndScheduleReturn {
        self.scheduler.schedule()
    }
}
