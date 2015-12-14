use ContinuationAdderMultipleTaskBoxesMultipleContinuationBoxes;
use ContinuationAdderTrait;
use EndScheduleMultipleTaskBoxesOneContinuationBox;
use EndScheduleTrait;
use SchedulerTrait;
use Task;
use TaskBox;
use TaskBoxIntoIterator;

pub struct ContinuationAdderMultipleTaskBoxesOneContinuationBox<'a,
                                                                TScheduler>
    where TScheduler: 'a +
                      SchedulerTrait {
    scheduler: &'a TScheduler,
    task_boxes: Vec<TaskBox<TScheduler::TTaskBoxParam>>,
    continuation_box: TaskBox<TScheduler::TTaskBoxParam>,
}

impl <'a,
      TScheduler> ContinuationAdderMultipleTaskBoxesOneContinuationBox<'a,
                                                                       TScheduler>
    where TScheduler: SchedulerTrait {
    pub fn new(scheduler: &'a TScheduler,
               task_boxes: Vec<TaskBox<TScheduler::TTaskBoxParam>>,
               continuation_box: TaskBox<TScheduler::TTaskBoxParam>) -> ContinuationAdderMultipleTaskBoxesOneContinuationBox<'a,
                                                                                                                             TScheduler>  {
        ContinuationAdderMultipleTaskBoxesOneContinuationBox { scheduler: scheduler,
                                                               task_boxes: task_boxes,
                                                               continuation_box: continuation_box }
    }

    fn convert_to_continuation_adder_multiple_tasks_multiple_continuations(self) -> ContinuationAdderMultipleTaskBoxesMultipleContinuationBoxes<'a,
                                                                                                                                                TScheduler> {
        let continuation_boxes = vec![self.continuation_box];
        ContinuationAdderMultipleTaskBoxesMultipleContinuationBoxes::new(self.scheduler,
                                                                         self.task_boxes,
                                                                         continuation_boxes)
    }

    fn convert_to_end_schedule_multiple_task_boxes_one_continuation_box(self) -> EndScheduleMultipleTaskBoxesOneContinuationBox<'a,
                                                                                                                                TScheduler> {
        EndScheduleMultipleTaskBoxesOneContinuationBox::new(self.scheduler,
                                                            self.task_boxes,
                                                            self.continuation_box)
    }
}

impl <'a,
      TScheduler> ContinuationAdderTrait<TScheduler,
                                         ContinuationAdderMultipleTaskBoxesMultipleContinuationBoxes<'a,
                                                                                                     TScheduler>,
                                         ContinuationAdderMultipleTaskBoxesMultipleContinuationBoxes<'a,
                                                                                                     TScheduler>> for ContinuationAdderMultipleTaskBoxesOneContinuationBox<'a,
                                                                                                                                                                           TScheduler>
    where TScheduler: SchedulerTrait {
    fn add_continuation<TTask>(self,
                               continuation: TTask) -> ContinuationAdderMultipleTaskBoxesMultipleContinuationBoxes<'a,
                                                                                                                   TScheduler>
        where TTask: 'static +
                     Task<TScheduler::TTaskBoxParam> {
        let continuation_box = Box::new(continuation);
        self.add_continuation_box(continuation_box)
    }

    fn add_continuation_box(self,
                            continuation_box: TaskBox<TScheduler::TTaskBoxParam>) -> ContinuationAdderMultipleTaskBoxesMultipleContinuationBoxes<'a,
                                                                                                                                                 TScheduler> {
        self.convert_to_continuation_adder_multiple_tasks_multiple_continuations()
            .add_continuation_box(continuation_box)
    }

    fn add_continuation_boxes<TTaskBoxIntoIterator>(self,
                                                    continuation_boxes: TTaskBoxIntoIterator) -> ContinuationAdderMultipleTaskBoxesMultipleContinuationBoxes<'a,
                                                                                                                                                             TScheduler>
        where TTaskBoxIntoIterator: TaskBoxIntoIterator<TScheduler::TTaskBoxParam> {
        self.convert_to_continuation_adder_multiple_tasks_multiple_continuations()
            .add_continuation_boxes(continuation_boxes)
    }
}

impl <'a,
      TScheduler> EndScheduleTrait for ContinuationAdderMultipleTaskBoxesOneContinuationBox<'a,
                                                                                            TScheduler>
    where TScheduler: SchedulerTrait {
    type TEndScheduleReturn = TScheduler::TScheduleMultipleReturn;

    fn end_schedule(self) -> Self::TEndScheduleReturn {
        self.convert_to_end_schedule_multiple_task_boxes_one_continuation_box()
            .end_schedule()
    }
}
