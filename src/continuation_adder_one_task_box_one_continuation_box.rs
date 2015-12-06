use ContinuationAdderOneTaskBoxMultipleContinuationBoxes;
use ContinuationAdderTrait;
use EndScheduleOneTaskBoxOneContinuationBox;
use EndScheduleTrait;
use SchedulerTrait;
use Task;
use TaskBox;
use TaskBoxIntoIterator;

pub struct ContinuationAdderOneTaskBoxOneContinuationBox<'a,
                                                         TScheduler>
    where TScheduler: 'a +
                      SchedulerTrait {
    scheduler: &'a TScheduler,
    task_box: TaskBox,
    continuation_box: TaskBox,
}

impl <'a,
      TScheduler> ContinuationAdderOneTaskBoxOneContinuationBox<'a,
                                                                TScheduler>
    where TScheduler: SchedulerTrait {
    pub fn new(scheduler: &'a TScheduler,
               task_box: TaskBox,
               continuation_box: TaskBox) -> ContinuationAdderOneTaskBoxOneContinuationBox<'a,
                                                                                           TScheduler>  {
        ContinuationAdderOneTaskBoxOneContinuationBox { scheduler: scheduler,
                                                        task_box: task_box,
                                                        continuation_box: continuation_box }
    }

    fn convert_to_continuation_adder_one_task_multiple_continuations(self) -> ContinuationAdderOneTaskBoxMultipleContinuationBoxes<'a,
                                                                                                                                   TScheduler> {
        ContinuationAdderOneTaskBoxMultipleContinuationBoxes::new(self.scheduler,
                                                                  self.task_box,
                                                                  Vec::new())
    }

    fn convert_to_end_schedule_one_task_box_one_continuation_box(self) -> EndScheduleOneTaskBoxOneContinuationBox<'a,
                                                                                                                  TScheduler> {
        EndScheduleOneTaskBoxOneContinuationBox::new(self.scheduler,
                                                     self.task_box,
                                                     self.continuation_box)
    }
}


impl <'a,
      TScheduler> ContinuationAdderTrait<ContinuationAdderOneTaskBoxMultipleContinuationBoxes<'a,
                                                                                              TScheduler>,
                                         ContinuationAdderOneTaskBoxMultipleContinuationBoxes<'a,
                                                                                              TScheduler>> for ContinuationAdderOneTaskBoxOneContinuationBox<'a,
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
        self.convert_to_continuation_adder_one_task_multiple_continuations()
            .add_continuation_box(continuation_box)
    }

    fn add_continuation_boxes<TTaskBoxIntoIterator>(self,
                                                    continuation_boxes: TTaskBoxIntoIterator) -> ContinuationAdderOneTaskBoxMultipleContinuationBoxes<'a,
                                                                                                                                                      TScheduler>
        where TTaskBoxIntoIterator: 'static +
                                    TaskBoxIntoIterator {
        self.convert_to_continuation_adder_one_task_multiple_continuations()
            .add_continuation_boxes(continuation_boxes)
    }
}

impl <'a,
      TScheduler> EndScheduleTrait for ContinuationAdderOneTaskBoxOneContinuationBox<'a,
                                                                                     TScheduler>
    where TScheduler: SchedulerTrait {
    type TEndScheduleReturn = TScheduler::TScheduleReturn;

    fn end_schedule(self) -> Self::TEndScheduleReturn {
        self.convert_to_end_schedule_one_task_box_one_continuation_box()
            .end_schedule()
    }
}
