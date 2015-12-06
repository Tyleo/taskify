use ContinuationAdderOneTaskBoxMultipleContinuationBoxes;
use ContinuationAdderTrait;
use EndScheduleOneTaskBoxOneContinuationBox;
use Scheduler;
use EndScheduleTrait;
use Task;
use TaskBox;
use TaskBoxIntoIterator;

pub struct ContinuationAdderOneTaskBoxOneContinuationBox<'a> {
    scheduler: &'a Scheduler,
    task_box: TaskBox,
    continuation_box: TaskBox,
}

impl <'a> ContinuationAdderOneTaskBoxOneContinuationBox<'a> {
    pub fn new(scheduler: &'a Scheduler,
               task_box: TaskBox,
               continuation_box: TaskBox) -> ContinuationAdderOneTaskBoxOneContinuationBox<'a>  {
        ContinuationAdderOneTaskBoxOneContinuationBox { scheduler: scheduler,
                                                        task_box: task_box,
                                                        continuation_box: continuation_box }
    }

    fn convert_to_continuation_adder_one_task_multiple_continuations(self) -> ContinuationAdderOneTaskBoxMultipleContinuationBoxes<'a> {
        ContinuationAdderOneTaskBoxMultipleContinuationBoxes::new(self.scheduler,
                                                                  self.task_box,
                                                                  Vec::new())
    }

    fn convert_to_end_schedule_one_task_box_one_continuation_box(self) -> EndScheduleOneTaskBoxOneContinuationBox<'a> {
        EndScheduleOneTaskBoxOneContinuationBox::new(self.scheduler,
                                                     self.task_box,
                                                     self.continuation_box)
    }
}


impl <'a> ContinuationAdderTrait<ContinuationAdderOneTaskBoxMultipleContinuationBoxes<'a>,
                                 ContinuationAdderOneTaskBoxMultipleContinuationBoxes<'a>> for ContinuationAdderOneTaskBoxOneContinuationBox<'a> {
    fn add_continuation<TTask>(self,
                               continuation: TTask) -> ContinuationAdderOneTaskBoxMultipleContinuationBoxes<'a>
        where TTask: 'static +
                     Task {
        let continuation_box = Box::new(continuation);
        self.add_continuation_box(continuation_box)
    }

    fn add_continuation_box(self,
                            continuation_box: TaskBox) -> ContinuationAdderOneTaskBoxMultipleContinuationBoxes<'a> {
        self.convert_to_continuation_adder_one_task_multiple_continuations()
            .add_continuation_box(continuation_box)
    }

    fn add_continuation_boxes<TTaskBoxIntoIterator>(self,
                                                    continuation_boxes: TTaskBoxIntoIterator) -> ContinuationAdderOneTaskBoxMultipleContinuationBoxes<'a>
        where TTaskBoxIntoIterator: 'static +
                                    TaskBoxIntoIterator {
        self.convert_to_continuation_adder_one_task_multiple_continuations()
            .add_continuation_boxes(continuation_boxes)
    }
}

impl <'a> EndScheduleTrait<()> for ContinuationAdderOneTaskBoxOneContinuationBox<'a> {
    fn end_schedule(self) {
        self.convert_to_end_schedule_one_task_box_one_continuation_box()
            .end_schedule()
    }
}
