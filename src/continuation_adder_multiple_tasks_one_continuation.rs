use ContinuationAdderMultipleTasksMultipleContinuations;
use ContinuationAdderTrait;
use LooseContinuation;
use LooseContinuationIntoIterator;
use ScheduleMultipleTasksOneContinuation;
use Scheduler;
use ScheduleTrait;
use Task;
use TaskBox;
use TaskBoxIntoIterator;

pub struct ContinuationAdderMultipleTasksOneContinuation<'a> {
    scheduler: &'a Scheduler,
    task_boxes: Vec<TaskBox>,
    continuation_box: TaskBox,
}

impl <'a> ContinuationAdderMultipleTasksOneContinuation<'a> {
    pub fn new(scheduler: &'a Scheduler,
               task_boxes: Vec<TaskBox>,
               continuation_box: TaskBox) -> ContinuationAdderMultipleTasksOneContinuation<'a>  {
        ContinuationAdderMultipleTasksOneContinuation { scheduler: scheduler,
                                                        task_boxes: task_boxes,
                                                        continuation_box: continuation_box }
    }

    fn convert_to_continuation_adder_multiple_tasks_multiple_continuations(self) -> ContinuationAdderMultipleTasksMultipleContinuations<'a> {
        let continuation_boxes = vec![self.continuation_box];
        ContinuationAdderMultipleTasksMultipleContinuations::new(self.scheduler,
                                                                 self.task_boxes,
                                                                 continuation_boxes)
    }

    fn convert_to_schedule(self) -> ScheduleMultipleTasksOneContinuation<'a> {
        ScheduleMultipleTasksOneContinuation::new(self.scheduler,
                                                  self.task_boxes,
                                                  self.continuation_box)
    }
}

impl <'a> ContinuationAdderTrait<ContinuationAdderMultipleTasksMultipleContinuations<'a>,
                                 ContinuationAdderMultipleTasksMultipleContinuations<'a>> for ContinuationAdderMultipleTasksOneContinuation<'a> {
    fn add_continuation<TTask>(self,
                               continuation: TTask) -> ContinuationAdderMultipleTasksMultipleContinuations<'a>
        where TTask: 'static +
                     Task {
        let continuation_box = Box::new(continuation);
        self.add_continuation_box(continuation_box)
    }

    fn add_continuation_box(self,
                            continuation_box: TaskBox) -> ContinuationAdderMultipleTasksMultipleContinuations<'a> {
        self.convert_to_continuation_adder_multiple_tasks_multiple_continuations()
            .add_continuation_box(continuation_box)
    }

    fn add_continuation_boxes<TTaskBoxIntoIterator>(self,
                                                    continuation_boxes: TTaskBoxIntoIterator) -> ContinuationAdderMultipleTasksMultipleContinuations<'a>
        where TTaskBoxIntoIterator: 'static +
                                    TaskBoxIntoIterator {
        self.convert_to_continuation_adder_multiple_tasks_multiple_continuations()
            .add_continuation_boxes(continuation_boxes)
    }

    // fn add_loose_continuation(self, loose_continuation: LooseContinuation) -> ContinuationAdderMultipleTasksMultipleContinuations<'a> {
    //     self.convert_to_continuation_adder_multiple_tasks_multiple_continuations()
    //         .add_loose_continuation(loose_continuation)
    // }

    // fn add_loose_continuations<TLooseContinuationIntoIterator: 'static + LooseContinuationIntoIterator>(self, loose_continuations: TLooseContinuationIntoIterator) -> ContinuationAdderMultipleTasksMultipleContinuations<'a> {
    //     self.convert_to_continuation_adder_multiple_tasks_multiple_continuations()
    //         .add_loose_continuations(loose_continuations)
    // }
}

impl <'a> ScheduleTrait for ContinuationAdderMultipleTasksOneContinuation<'a> {
    fn schedule(self) {
        self.convert_to_schedule()
            .schedule()
    }
}
