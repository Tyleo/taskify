use ContinuationAdderTrait;
use ContinuationAdderMultipleTasksMultipleContinuations;
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
}

impl <'a> ContinuationAdderMultipleTasksOneContinuation<'a> {
    pub fn new(scheduler: &'a Scheduler) -> ContinuationAdderMultipleTasksOneContinuation<'a>  {
        ContinuationAdderMultipleTasksOneContinuation { scheduler: scheduler }
    }

    fn convert_to_continuation_adder_multiple_tasks_multiple_continuations(self) -> ContinuationAdderMultipleTasksMultipleContinuations<'a> {
        ContinuationAdderMultipleTasksMultipleContinuations::new(self.scheduler)
    }

    fn convert_to_schedule(self) -> ScheduleMultipleTasksOneContinuation {
        ScheduleMultipleTasksOneContinuation
    }
}

impl <'a> ContinuationAdderTrait<ContinuationAdderMultipleTasksMultipleContinuations<'a>,
                                 ContinuationAdderMultipleTasksMultipleContinuations<'a>> for ContinuationAdderMultipleTasksOneContinuation<'a> {
    fn add_continuation<TTask: 'static + Task>(self, continuation: TTask) -> ContinuationAdderMultipleTasksMultipleContinuations<'a> {
        self.convert_to_continuation_adder_multiple_tasks_multiple_continuations()
            .add_continuation(continuation)
    }

    fn add_continuation_box(self, continuation_box: TaskBox) -> ContinuationAdderMultipleTasksMultipleContinuations<'a> {
        self.convert_to_continuation_adder_multiple_tasks_multiple_continuations()
            .add_continuation_box(continuation_box)
    }

    fn add_continuation_boxes<TTaskBoxIntoIterator: 'static + TaskBoxIntoIterator>(self, continuation_boxes: TTaskBoxIntoIterator) -> ContinuationAdderMultipleTasksMultipleContinuations<'a> {
        self.convert_to_continuation_adder_multiple_tasks_multiple_continuations()
            .add_continuation_boxes(continuation_boxes)
    }

    fn add_loose_continuation(self, loose_continuation: LooseContinuation) -> ContinuationAdderMultipleTasksMultipleContinuations<'a> {
        self.convert_to_continuation_adder_multiple_tasks_multiple_continuations()
            .add_loose_continuation(loose_continuation)
    }

    fn add_loose_continuations<TLooseContinuationIntoIterator: 'static + LooseContinuationIntoIterator>(self, loose_continuations: TLooseContinuationIntoIterator) -> ContinuationAdderMultipleTasksMultipleContinuations<'a> {
        self.convert_to_continuation_adder_multiple_tasks_multiple_continuations()
            .add_loose_continuations(loose_continuations)
    }
}

impl <'a> ScheduleTrait for ContinuationAdderMultipleTasksOneContinuation<'a> {
    fn schedule(self) {
        self.convert_to_schedule()
            .schedule()
    }
}
