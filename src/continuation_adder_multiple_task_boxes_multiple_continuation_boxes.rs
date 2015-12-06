use ContinuationAdderTrait;
use EndScheduleMultipleTaskBoxesMultipleContinuationBoxes;
use Scheduler;
use EndScheduleTrait;
use Task;
use TaskBox;
use TaskBoxIntoIterator;

pub struct ContinuationAdderMultipleTaskBoxesMultipleContinuationBoxes<'a> {
    scheduler: &'a Scheduler,
    task_boxes: Vec<TaskBox>,
    continuation_boxes: Vec<TaskBox>,
}

impl <'a> ContinuationAdderMultipleTaskBoxesMultipleContinuationBoxes<'a> {
    pub fn new(scheduler: &'a Scheduler,
               task_boxes: Vec<TaskBox>,
               continuation_boxes: Vec<TaskBox>) -> ContinuationAdderMultipleTaskBoxesMultipleContinuationBoxes<'a> {
        ContinuationAdderMultipleTaskBoxesMultipleContinuationBoxes { scheduler: scheduler,
                                                                      task_boxes: task_boxes,
                                                                      continuation_boxes: continuation_boxes }
    }

    fn convert_to_end_schedule_multiple_task_boxes_multiple_continuation_boxes(self) -> EndScheduleMultipleTaskBoxesMultipleContinuationBoxes<'a> {
        EndScheduleMultipleTaskBoxesMultipleContinuationBoxes::new(self.scheduler,
                                                               self.task_boxes,
                                                               self.continuation_boxes)
    }
}

impl <'a> ContinuationAdderTrait<ContinuationAdderMultipleTaskBoxesMultipleContinuationBoxes<'a>,
                                 ContinuationAdderMultipleTaskBoxesMultipleContinuationBoxes<'a>> for ContinuationAdderMultipleTaskBoxesMultipleContinuationBoxes<'a> {
    fn add_continuation<TTask>(self,
                               continuation: TTask) -> ContinuationAdderMultipleTaskBoxesMultipleContinuationBoxes<'a>
        where TTask: 'static +
                     Task {
        let continuation_box = Box::new(continuation);
        self.add_continuation_box(continuation_box)
    }

    fn add_continuation_box(self,
                            continuation_box: TaskBox) -> ContinuationAdderMultipleTaskBoxesMultipleContinuationBoxes<'a> {
        let mut mut_self = self;
        mut_self.continuation_boxes.push(continuation_box);
        mut_self
    }

    fn add_continuation_boxes<TTaskBoxIntoIterator>(self,
                                                    continuation_boxes: TTaskBoxIntoIterator) -> ContinuationAdderMultipleTaskBoxesMultipleContinuationBoxes<'a>
        where TTaskBoxIntoIterator: 'static +
                                    TaskBoxIntoIterator {
        let mut mut_self = self;
        for continuation_box in continuation_boxes {
            mut_self.continuation_boxes.push(continuation_box);
        }
        mut_self
    }
}

impl <'a> EndScheduleTrait<()> for ContinuationAdderMultipleTaskBoxesMultipleContinuationBoxes<'a> {
    fn end_schedule(self) {
        self.convert_to_end_schedule_multiple_task_boxes_multiple_continuation_boxes()
            .end_schedule()
    }
}
