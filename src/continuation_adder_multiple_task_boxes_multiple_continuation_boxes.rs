use ContinuationAdderTrait;
use EndScheduleMultipleTasksMultipleContinuations;
use Scheduler;
use EndScheduleTrait;
use Task;
use TaskBox;
use TaskBoxIntoIterator;

pub struct ContinuationAdderMultipleTasksMultipleContinuations<'a> {
    scheduler: &'a Scheduler,
    task_boxes: Vec<TaskBox>,
    continuation_boxes: Vec<TaskBox>,
}

impl <'a> ContinuationAdderMultipleTasksMultipleContinuations<'a> {
    pub fn new(scheduler: &'a Scheduler,
               task_boxes: Vec<TaskBox>,
               continuation_boxes: Vec<TaskBox>) -> ContinuationAdderMultipleTasksMultipleContinuations<'a> {
        ContinuationAdderMultipleTasksMultipleContinuations { scheduler: scheduler,
                                                              task_boxes: task_boxes,
                                                              continuation_boxes: continuation_boxes }
    }

    fn convert_to_end_schedule_multiple_task_boxes_multiple_continuation_boxes(self) -> EndScheduleMultipleTasksMultipleContinuations<'a> {
        EndScheduleMultipleTasksMultipleContinuations::new(self.scheduler,
                                                           self.task_boxes,
                                                           self.continuation_boxes)
    }
}

impl <'a> ContinuationAdderTrait<ContinuationAdderMultipleTasksMultipleContinuations<'a>,
                                 ContinuationAdderMultipleTasksMultipleContinuations<'a>> for ContinuationAdderMultipleTasksMultipleContinuations<'a> {
    fn add_continuation<TTask>(self,
                               continuation: TTask) -> ContinuationAdderMultipleTasksMultipleContinuations<'a>
        where TTask: 'static +
                     Task {
        let continuation_box = Box::new(continuation);
        self.add_continuation_box(continuation_box)
    }

    fn add_continuation_box(self,
                            continuation_box: TaskBox) -> ContinuationAdderMultipleTasksMultipleContinuations<'a> {
        let mut mut_self = self;
        mut_self.continuation_boxes.push(continuation_box);
        mut_self
    }

    fn add_continuation_boxes<TTaskBoxIntoIterator>(self,
                                                    continuation_boxes: TTaskBoxIntoIterator) -> ContinuationAdderMultipleTasksMultipleContinuations<'a>
        where TTaskBoxIntoIterator: 'static +
                                    TaskBoxIntoIterator {
        let mut mut_self = self;
        for continuation_box in continuation_boxes {
            mut_self.continuation_boxes.push(continuation_box);
        }
        mut_self
    }
}

impl <'a> EndScheduleTrait<()> for ContinuationAdderMultipleTasksMultipleContinuations<'a> {
    fn end_schedule(self) {
        self.convert_to_end_schedule_multiple_task_boxes_multiple_continuation_boxes()
            .end_schedule()
    }
}
