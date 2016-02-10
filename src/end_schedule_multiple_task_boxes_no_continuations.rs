use fluent::EndScheduleTrait;
use SchedulerTrait;
use TaskBox;

pub struct EndScheduleMultipleTaskBoxesNoContinuations<'a,
                                                       TScheduler>
    where TScheduler: 'a +
                      SchedulerTrait {
    scheduler: &'a TScheduler,
    task_boxes: Vec<TaskBox<TScheduler::TTaskBoxParam>>,
}

impl <'a,
      TScheduler> EndScheduleMultipleTaskBoxesNoContinuations<'a,
                                                              TScheduler>
    where TScheduler: SchedulerTrait {
    pub fn new(scheduler: &'a TScheduler,
               task_boxes: Vec<TaskBox<TScheduler::TTaskBoxParam>>) -> EndScheduleMultipleTaskBoxesNoContinuations<'a,
                                                                                                                   TScheduler> {
        EndScheduleMultipleTaskBoxesNoContinuations { scheduler: scheduler,
                                                      task_boxes: task_boxes }
    }
}

impl <'a,
      TScheduler> EndScheduleTrait for EndScheduleMultipleTaskBoxesNoContinuations<'a,
                                                                                   TScheduler>
    where TScheduler: SchedulerTrait {
    type TEndScheduleReturn = TScheduler::TScheduleMultipleReturn;

    fn end_schedule(self) -> Self::TEndScheduleReturn {
        let task_boxes = self.task_boxes;

        self.scheduler.schedule_multiple(task_boxes)
    }
}
