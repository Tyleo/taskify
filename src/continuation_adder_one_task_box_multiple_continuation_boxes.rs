use ContinuationAdderTrait;
use EndScheduleOneTaskBoxMultipleContinuationBoxes;
use EndScheduleTrait;
use SchedulerTrait;
use Task;
use TaskBox;
use TaskBoxIntoIterator;

pub struct ContinuationAdderOneTaskBoxMultipleContinuationBoxes<'a,
                                                                TScheduler>
    where TScheduler: 'a +
                      SchedulerTrait {
    scheduler: &'a TScheduler,
    task_box: TaskBox,
    continuation_boxes: Vec<TaskBox>,
}

impl <'a,
      TScheduler> ContinuationAdderOneTaskBoxMultipleContinuationBoxes<'a,
                                                                       TScheduler>
    where TScheduler: SchedulerTrait {
    pub fn new(scheduler: &'a TScheduler,
               task_box: TaskBox,
               continuation_boxes: Vec<TaskBox>) -> ContinuationAdderOneTaskBoxMultipleContinuationBoxes<'a,
                                                                                                         TScheduler>  {
        ContinuationAdderOneTaskBoxMultipleContinuationBoxes { scheduler: scheduler,
                                                               task_box: task_box,
                                                               continuation_boxes: continuation_boxes }
    }

    fn convert_to_end_schedule_one_task_box_multiple_continuation_boxes(self) -> EndScheduleOneTaskBoxMultipleContinuationBoxes<'a,
                                                                                                                                TScheduler> {
        EndScheduleOneTaskBoxMultipleContinuationBoxes::new(self.scheduler,
                                                     self.task_box,
                                                     self.continuation_boxes)
    }
}

impl <'a,
      TScheduler> ContinuationAdderTrait<ContinuationAdderOneTaskBoxMultipleContinuationBoxes<'a,
                                                                                              TScheduler>,
                                         ContinuationAdderOneTaskBoxMultipleContinuationBoxes<'a,
                                                                                              TScheduler>> for ContinuationAdderOneTaskBoxMultipleContinuationBoxes<'a,
                                                                                                                                                                    TScheduler>
    where TScheduler: SchedulerTrait {
    fn add_continuation<TTask>(self,
                               continuation: TTask) -> ContinuationAdderOneTaskBoxMultipleContinuationBoxes<'a,
                                                                                                            TScheduler>
        where TTask: 'static +
                     Task {
        let continuation_box = Box::new(continuation);
        self.add_continuation_box(continuation_box)
    }

    fn add_continuation_box(self,
                            continuation_box: TaskBox) -> ContinuationAdderOneTaskBoxMultipleContinuationBoxes<'a,
                                                                                                               TScheduler> {
        let mut mut_self = self;
        mut_self.continuation_boxes.push(continuation_box);
        mut_self
    }

    fn add_continuation_boxes<TTaskBoxIntoIterator>(self,
                                                    continuation_boxes: TTaskBoxIntoIterator) -> ContinuationAdderOneTaskBoxMultipleContinuationBoxes<'a,
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
      TScheduler> EndScheduleTrait for ContinuationAdderOneTaskBoxMultipleContinuationBoxes<'a,
                                                                                            TScheduler>
    where TScheduler: SchedulerTrait {
    type TEndScheduleReturn = TScheduler::TScheduleReturn;

    fn end_schedule(self) -> Self::TEndScheduleReturn {
        self.convert_to_end_schedule_one_task_box_multiple_continuation_boxes()
            .end_schedule()
    }
}
