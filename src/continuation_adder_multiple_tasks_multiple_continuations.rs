use ContinuationAdderTrait;
use LooseContinuation;
use LooseContinuationIntoIterator;
use ScheduleMultipleTasksMultipleContinuations;
use Scheduler;
use ScheduleTrait;
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

    fn convert_to_schedule(self) -> ScheduleMultipleTasksMultipleContinuations<'a> {
        ScheduleMultipleTasksMultipleContinuations::new(self.scheduler)
    }
}

impl <'a> ContinuationAdderTrait<ContinuationAdderMultipleTasksMultipleContinuations<'a>,
                                 ContinuationAdderMultipleTasksMultipleContinuations<'a>> for ContinuationAdderMultipleTasksMultipleContinuations<'a> {
    fn add_continuation<TTask: 'static + Task>(self, continuation: TTask) -> ContinuationAdderMultipleTasksMultipleContinuations<'a> {
        let continuation_box = Box::new(continuation);
        self.add_continuation_box(continuation_box)
    }

    fn add_continuation_box(self, continuation_box: TaskBox) -> ContinuationAdderMultipleTasksMultipleContinuations<'a> {
        let mut mut_self = self;
        mut_self.continuation_boxes.push(continuation_box);
        mut_self
    }

    fn add_continuation_boxes<TTaskBoxIntoIterator: 'static + TaskBoxIntoIterator>(self, continuation_boxes: TTaskBoxIntoIterator) -> ContinuationAdderMultipleTasksMultipleContinuations<'a> {
        let mut mut_self = self;
        for continuation_box in continuation_boxes {
            mut_self.continuation_boxes.push(continuation_box);
        }
        mut_self
    }

    // fn add_loose_continuation(self, loose_continuation: LooseContinuation) -> ContinuationAdderMultipleTasksMultipleContinuations<'a> {
    //     self
    // }

    // fn add_loose_continuations<TLooseContinuationIntoIterator: 'static + LooseContinuationIntoIterator>(self, loose_continuations: TLooseContinuationIntoIterator) -> ContinuationAdderMultipleTasksMultipleContinuations<'a> {
    //     self
    // }
}

impl <'a> ScheduleTrait for ContinuationAdderMultipleTasksMultipleContinuations<'a> {
    fn schedule(self) {
        self.convert_to_schedule()
            .schedule()
    }
}
