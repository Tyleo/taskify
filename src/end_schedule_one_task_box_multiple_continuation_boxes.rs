use fluent::EndScheduleTrait;
use Scheduler;
use SchedulerTrait;
use TaskBox;

pub struct EndScheduleOneTaskBoxMultipleContinuationBoxes<'a,
                                                          TScheduler>
    where TScheduler: 'a +
                      SchedulerTrait {
    scheduler: &'a TScheduler,
    task_box: TaskBox<TScheduler::TTaskBoxParam>,
    continuation_boxes: Vec<TaskBox<TScheduler::TTaskBoxParam>>,
}

impl <'a,
      TScheduler> EndScheduleOneTaskBoxMultipleContinuationBoxes<'a,
                                                                 TScheduler>
    where TScheduler: SchedulerTrait {
    pub fn new(scheduler: &'a TScheduler,
               task_box: TaskBox<TScheduler::TTaskBoxParam>,
               continuation_boxes: Vec<TaskBox<TScheduler::TTaskBoxParam>>) -> EndScheduleOneTaskBoxMultipleContinuationBoxes<'a,
                                                                                                                              TScheduler> {
        EndScheduleOneTaskBoxMultipleContinuationBoxes { scheduler: scheduler,
                                                         task_box: task_box,
                                                         continuation_boxes: continuation_boxes }
    }
}

impl <'a,
      TScheduler> EndScheduleTrait for EndScheduleOneTaskBoxMultipleContinuationBoxes<'a,
                                                                                      TScheduler>
    where TScheduler: SchedulerTrait {
    type TEndScheduleReturn = TScheduler::TScheduleReturn;

    fn end_schedule(self) -> Self::TEndScheduleReturn {
        let task_box = self.task_box;
        let continuation_boxes = self.continuation_boxes;

        let result_task = move |scheduler: &TScheduler::TTaskBoxParam| {
            task_box.call_box((&scheduler,));
            scheduler.schedule_multiple(continuation_boxes);
        };
        let result_task_box = Box::new(result_task);

        self.scheduler.schedule(result_task_box)
    }
}
