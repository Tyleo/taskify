use ContinuationAdderTrait;
use EndScheduleMultipleTaskBoxesMultipleContinuationBoxes;
use EndScheduleTrait;
use SchedulerTrait;
use Task;
use TaskBox;
use TaskBoxIntoIterator;

pub struct ContinuationAdderMultipleTaskBoxesMultipleContinuationBoxes<'a,
                                                                       TScheduler>
    where TScheduler: 'a +
                      SchedulerTrait {
    scheduler: &'a TScheduler,
    task_boxes: Vec<TaskBox>,
    continuation_boxes: Vec<TaskBox>,
}

impl <'a,
      TScheduler> ContinuationAdderMultipleTaskBoxesMultipleContinuationBoxes<'a,
                                                                              TScheduler>
    where TScheduler: SchedulerTrait {
    pub fn new(scheduler: &'a TScheduler,
               task_boxes: Vec<TaskBox>,
               continuation_boxes: Vec<TaskBox>) -> ContinuationAdderMultipleTaskBoxesMultipleContinuationBoxes<'a,
                                                                                                                TScheduler> {
        ContinuationAdderMultipleTaskBoxesMultipleContinuationBoxes { scheduler: scheduler,
                                                                      task_boxes: task_boxes,
                                                                      continuation_boxes: continuation_boxes }
    }

    fn convert_to_end_schedule_multiple_task_boxes_multiple_continuation_boxes(self) -> EndScheduleMultipleTaskBoxesMultipleContinuationBoxes<'a,
                                                                                                                                              TScheduler> {
        EndScheduleMultipleTaskBoxesMultipleContinuationBoxes::new(self.scheduler,
                                                                   self.task_boxes,
                                                                   self.continuation_boxes)
    }
}

impl <'a,
      TScheduler> ContinuationAdderTrait<ContinuationAdderMultipleTaskBoxesMultipleContinuationBoxes<'a,
                                                                                                     TScheduler>,
                                         ContinuationAdderMultipleTaskBoxesMultipleContinuationBoxes<'a,
                                                                                                     TScheduler>> for ContinuationAdderMultipleTaskBoxesMultipleContinuationBoxes<'a,
                                                                                                                                                                                  TScheduler>
    where TScheduler: SchedulerTrait {
    fn add_continuation<TTask>(self,
                               continuation: TTask) -> ContinuationAdderMultipleTaskBoxesMultipleContinuationBoxes<'a,
                                                                                                                   TScheduler>
        where TTask: 'static +
                     Task {
        let continuation_box = Box::new(continuation);
        self.add_continuation_box(continuation_box)
    }

    fn add_continuation_box(self,
                            continuation_box: TaskBox) -> ContinuationAdderMultipleTaskBoxesMultipleContinuationBoxes<'a,
                                                                                                                      TScheduler> {
        let mut mut_self = self;
        mut_self.continuation_boxes.push(continuation_box);
        mut_self
    }

    fn add_continuation_boxes<TTaskBoxIntoIterator>(self,
                                                    continuation_boxes: TTaskBoxIntoIterator) -> ContinuationAdderMultipleTaskBoxesMultipleContinuationBoxes<'a,
                                                                                                                                                             TScheduler>
        where TTaskBoxIntoIterator: TaskBoxIntoIterator {
        let mut mut_self = self;
        for continuation_box in continuation_boxes {
            mut_self.continuation_boxes.push(continuation_box);
        }
        mut_self
    }
}

impl <'a,
      TScheduler> EndScheduleTrait for ContinuationAdderMultipleTaskBoxesMultipleContinuationBoxes<'a,
                                                                                                   TScheduler>
    where TScheduler: SchedulerTrait {
    type TEndScheduleReturn = TScheduler::TScheduleMultipleReturn;

    fn end_schedule(self) -> Self::TEndScheduleReturn {
        self.convert_to_end_schedule_multiple_task_boxes_multiple_continuation_boxes()
            .end_schedule()
    }
}
