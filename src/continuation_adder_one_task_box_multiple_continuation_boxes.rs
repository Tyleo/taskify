use ContinuationAdderTrait;
use EndScheduleOneTaskBoxMultipleContinuationBoxes;
use Scheduler;
use EndScheduleTrait;
use Task;
use TaskBox;
use TaskBoxIntoIterator;

pub struct ContinuationAdderOneTaskBoxMultipleContinuationBoxes<'a> {
    scheduler: &'a Scheduler,
    task_box: TaskBox,
    continuation_boxes: Vec<TaskBox>,
}

impl <'a> ContinuationAdderOneTaskBoxMultipleContinuationBoxes<'a> {
    pub fn new(scheduler: &'a Scheduler,
               task_box: TaskBox,
               continuation_boxes: Vec<TaskBox>) -> ContinuationAdderOneTaskBoxMultipleContinuationBoxes<'a>  {
        ContinuationAdderOneTaskBoxMultipleContinuationBoxes { scheduler: scheduler,
                                                               task_box: task_box,
                                                               continuation_boxes: continuation_boxes }
    }

    fn convert_to_end_schedule_one_task_box_multiple_continuation_boxes(self) -> EndScheduleOneTaskBoxMultipleContinuationBoxes<'a> {
        EndScheduleOneTaskBoxMultipleContinuationBoxes::new(self.scheduler,
                                                     self.task_box,
                                                     self.continuation_boxes)
    }
}

impl <'a> ContinuationAdderTrait<ContinuationAdderOneTaskBoxMultipleContinuationBoxes<'a>,
                                 ContinuationAdderOneTaskBoxMultipleContinuationBoxes<'a>> for ContinuationAdderOneTaskBoxMultipleContinuationBoxes<'a> {
    fn add_continuation<TTask>(self,
                               continuation: TTask) -> ContinuationAdderOneTaskBoxMultipleContinuationBoxes<'a>
        where TTask: 'static +
                     Task {
        let continuation_box = Box::new(continuation);
        self.add_continuation_box(continuation_box)
    }

    fn add_continuation_box(self,
                            continuation_box: TaskBox) -> ContinuationAdderOneTaskBoxMultipleContinuationBoxes<'a> {
        let mut mut_self = self;
        mut_self.continuation_boxes.push(continuation_box);
        mut_self
    }

    fn add_continuation_boxes<TTaskBoxIntoIterator>(self,
                                                    continuation_boxes: TTaskBoxIntoIterator) -> ContinuationAdderOneTaskBoxMultipleContinuationBoxes<'a>
        where TTaskBoxIntoIterator: 'static +
                                    TaskBoxIntoIterator {
        let mut mut_self = self;
        for continuation_box in continuation_boxes {
            mut_self.continuation_boxes.push(continuation_box);
        }
        mut_self
    }
}

impl <'a> EndScheduleTrait<()> for ContinuationAdderOneTaskBoxMultipleContinuationBoxes<'a> {
    fn end_schedule(self) {
        self.convert_to_end_schedule_one_task_box_multiple_continuation_boxes()
            .end_schedule()
    }
}
