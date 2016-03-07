use DecayPtr;
use fluent::EndScheduleTrait;
use SchedulerTrait;
use TaskBox;

pub struct EndScheduleMultipleTaskBoxesMultipleContinuationBoxes<'a,
                                                                 TScheduler>
    where TScheduler: 'a +
                      SchedulerTrait {
    scheduler: &'a TScheduler,
    task_boxes: Vec<TaskBox<TScheduler::TTaskBoxParam>>,
    continuation_boxes: Vec<TaskBox<TScheduler::TTaskBoxParam>>,
}

impl <'a,
      TScheduler> EndScheduleMultipleTaskBoxesMultipleContinuationBoxes<'a,
                                                                        TScheduler>
    where TScheduler: SchedulerTrait {
    pub fn new(scheduler: &'a TScheduler,
               task_boxes: Vec<TaskBox<TScheduler::TTaskBoxParam>>,
               continuation_boxes: Vec<TaskBox<TScheduler::TTaskBoxParam>>) -> EndScheduleMultipleTaskBoxesMultipleContinuationBoxes<'a,
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
    type TEndScheduleReturn = TScheduler::TScheduleMultipleReturn;

    fn end_schedule(self) -> Self::TEndScheduleReturn {
        let task_boxes = self.task_boxes;
        let continuation_boxes = self.continuation_boxes;

        let decaying_continuation_boxes = DecayPtr::new(continuation_boxes);

        let result_tasks: Vec<_> =
            task_boxes
                .into_iter()
                .map(
                    |task_box: TaskBox<TScheduler::TTaskBoxParam>| -> TaskBox<TScheduler::TTaskBoxParam> {
                        let decaying_continuation_boxes_clone = decaying_continuation_boxes.clone();

                        Box::new(
                            move |scheduler: &TScheduler::TTaskBoxParam| {
                                task_box.call_box((&scheduler,));
                                if let Some(continuation_boxes) = decaying_continuation_boxes_clone.decay() {
                                    scheduler.schedule_multiple(continuation_boxes);
                                }
                            }
                        )
                    }
                ).collect();

        decaying_continuation_boxes.decay();

        self.scheduler.schedule_multiple(result_tasks)
    }
}
