use ContinuationAdderMultipleTaskBoxesMultipleContinuationBoxes;
use ContinuationAdderTrait;
use EndScheduleMultipleTaskBoxesOneContinuationBox;
use Scheduler;
use EndScheduleTrait;
use Task;
use TaskBox;
use TaskBoxIntoIterator;

pub struct ContinuationAdderMultipleTaskBoxesOneContinuationBox<'a> {
    scheduler: &'a Scheduler,
    task_boxes: Vec<TaskBox>,
    continuation_box: TaskBox,
}

impl <'a> ContinuationAdderMultipleTaskBoxesOneContinuationBox<'a> {
    pub fn new(scheduler: &'a Scheduler,
               task_boxes: Vec<TaskBox>,
               continuation_box: TaskBox) -> ContinuationAdderMultipleTaskBoxesOneContinuationBox<'a>  {
        ContinuationAdderMultipleTaskBoxesOneContinuationBox { scheduler: scheduler,
                                                               task_boxes: task_boxes,
                                                               continuation_box: continuation_box }
    }

    fn convert_to_continuation_adder_multiple_tasks_multiple_continuations(self) -> ContinuationAdderMultipleTaskBoxesMultipleContinuationBoxes<'a> {
        let continuation_boxes = vec![self.continuation_box];
        ContinuationAdderMultipleTaskBoxesMultipleContinuationBoxes::new(self.scheduler,
                                                                         self.task_boxes,
                                                                         continuation_boxes)
    }

    fn convert_to_end_schedule_multiple_task_boxes_one_continuation_box(self) -> EndScheduleMultipleTaskBoxesOneContinuationBox<'a> {
        EndScheduleMultipleTaskBoxesOneContinuationBox::new(self.scheduler,
                                                            self.task_boxes,
                                                            self.continuation_box)
    }
}

impl <'a> ContinuationAdderTrait<ContinuationAdderMultipleTaskBoxesMultipleContinuationBoxes<'a>,
                                 ContinuationAdderMultipleTaskBoxesMultipleContinuationBoxes<'a>> for ContinuationAdderMultipleTaskBoxesOneContinuationBox<'a> {
    fn add_continuation<TTask>(self,
                               continuation: TTask) -> ContinuationAdderMultipleTaskBoxesMultipleContinuationBoxes<'a>
        where TTask: 'static +
                     Task {
        let continuation_box = Box::new(continuation);
        self.add_continuation_box(continuation_box)
    }

    fn add_continuation_box(self,
                            continuation_box: TaskBox) -> ContinuationAdderMultipleTaskBoxesMultipleContinuationBoxes<'a> {
        self.convert_to_continuation_adder_multiple_tasks_multiple_continuations()
            .add_continuation_box(continuation_box)
    }

    fn add_continuation_boxes<TTaskBoxIntoIterator>(self,
                                                    continuation_boxes: TTaskBoxIntoIterator) -> ContinuationAdderMultipleTaskBoxesMultipleContinuationBoxes<'a>
        where TTaskBoxIntoIterator: 'static +
                                    TaskBoxIntoIterator {
        self.convert_to_continuation_adder_multiple_tasks_multiple_continuations()
            .add_continuation_boxes(continuation_boxes)
    }
}

impl <'a> EndScheduleTrait<()> for ContinuationAdderMultipleTaskBoxesOneContinuationBox<'a> {
    fn end_schedule(self) {
        self.convert_to_end_schedule_multiple_task_boxes_one_continuation_box()
            .end_schedule()
    }
}
