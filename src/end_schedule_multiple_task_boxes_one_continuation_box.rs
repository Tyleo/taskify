use DecayPtr;
use fluent::EndScheduleTrait;
use SchedulerTrait;
use TaskBox;

pub struct EndScheduleMultipleTaskBoxesOneContinuationBox<'a,
                                                          TScheduler>
    where TScheduler: 'a +
                      SchedulerTrait {
    scheduler: &'a TScheduler,
    task_boxes: Vec<TaskBox<TScheduler::TTaskBoxParam>>,
    continuation_box: TaskBox<TScheduler::TTaskBoxParam>,
}

impl <'a,
      TScheduler> EndScheduleMultipleTaskBoxesOneContinuationBox<'a,
                                                                 TScheduler>
    where TScheduler: SchedulerTrait {
    pub fn new(scheduler: &'a TScheduler,
               task_boxes: Vec<TaskBox<TScheduler::TTaskBoxParam>>,
               continuation_box: TaskBox<TScheduler::TTaskBoxParam>) -> EndScheduleMultipleTaskBoxesOneContinuationBox<'a,
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
    type TEndScheduleReturn = TScheduler::TScheduleMultipleReturn;

    fn end_schedule(self) -> Self::TEndScheduleReturn {
        let task_boxes = self.task_boxes;
        let continuation_box = self.continuation_box;

        let decaying_continuation_box = DecayPtr::new(continuation_box);

        let result_tasks: Vec<_> =
            task_boxes
                .into_iter()
                .map(
                    |task_box: TaskBox<TScheduler::TTaskBoxParam>| -> TaskBox<TScheduler::TTaskBoxParam> {
                        let decaying_continuation_box_clone = decaying_continuation_box.clone();

                        Box::new(
                            move |scheduler: &TScheduler::TTaskBoxParam| {
                                task_box.call_box((&scheduler,));
                                if let Some(continuation_box) = decaying_continuation_box_clone.decay() {
                                    scheduler.schedule(continuation_box);
                                }
                            }
                        )
                    }
                ).collect();

        decaying_continuation_box.decay();

        self.scheduler.schedule_multiple(result_tasks)
    }
}
