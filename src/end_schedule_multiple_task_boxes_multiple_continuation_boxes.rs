use DecayPtr;
use fluent::EndScheduleTrait;
use Scheduler;
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

        let mut result_tasks = Vec::<TaskBox<TScheduler::TTaskBoxParam>>::new();

        for task_box in task_boxes {
            let current_decaying_continuation_boxes = unsafe { decaying_continuation_boxes.clone() };

            let current_task = move |scheduler: &TScheduler::TTaskBoxParam| {
                task_box.call_box((&scheduler,));
                match unsafe { current_decaying_continuation_boxes.decay() } {
                    Some(continuation_boxes) => {
                        scheduler.schedule_multiple(continuation_boxes);
                    },
                    None => {
                        // Do nothing
                    },
                }
            };
            let current_task_box = Box::new(current_task);

            result_tasks.push(current_task_box);
        }

        decaying_continuation_boxes.decay();

        self.scheduler.schedule_multiple(result_tasks)
    }
}
