use ContinuationAdderTrait;
use ContinuationAdderMultipleTasksMultipleContinuations;
use LooseContinuation;
use LooseContinuationIntoIterator;
use ScheduleMultipleTasksOneContinuation;
use ScheduleTrait;
use Task;
use TaskBox;
use TaskBoxIntoIterator;

pub struct ContinuationAdderMultipleTasksOneContinuation;

impl ContinuationAdderMultipleTasksOneContinuation {
    fn convert_to_continuation_adder_multiple_tasks_multiple_continuations(self) -> ContinuationAdderMultipleTasksMultipleContinuations {
        ContinuationAdderMultipleTasksMultipleContinuations
    }

    fn convert_to_schedule(self) -> ScheduleMultipleTasksOneContinuation {
        ScheduleMultipleTasksOneContinuation
    }
}

impl ContinuationAdderTrait<ContinuationAdderMultipleTasksMultipleContinuations,
                            ContinuationAdderMultipleTasksMultipleContinuations> for ContinuationAdderMultipleTasksOneContinuation {
    fn add_continuation<TTask: 'static + Task>(self, continuation: TTask) -> ContinuationAdderMultipleTasksMultipleContinuations {
        self.convert_to_continuation_adder_multiple_tasks_multiple_continuations()
            .add_continuation(continuation)
    }

    fn add_continuation_box(self, continuation_box: TaskBox) -> ContinuationAdderMultipleTasksMultipleContinuations {
        self.convert_to_continuation_adder_multiple_tasks_multiple_continuations()
            .add_continuation_box(continuation_box)
    }

    fn add_continuation_boxes<TTaskBoxIntoIterator: 'static + TaskBoxIntoIterator>(self, continuation_boxes: TTaskBoxIntoIterator) -> ContinuationAdderMultipleTasksMultipleContinuations {
        self.convert_to_continuation_adder_multiple_tasks_multiple_continuations()
            .add_continuation_boxes(continuation_boxes)
    }

    fn add_loose_continuation(self, loose_continuation: LooseContinuation) -> ContinuationAdderMultipleTasksMultipleContinuations {
        self.convert_to_continuation_adder_multiple_tasks_multiple_continuations()
            .add_loose_continuation(loose_continuation)
    }

    fn add_loose_continuations<TLooseContinuationIntoIterator: 'static + LooseContinuationIntoIterator>(self, loose_continuations: TLooseContinuationIntoIterator) -> ContinuationAdderMultipleTasksMultipleContinuations {
        self.convert_to_continuation_adder_multiple_tasks_multiple_continuations()
            .add_loose_continuations(loose_continuations)
    }
}

impl ScheduleTrait for ContinuationAdderMultipleTasksOneContinuation {
    fn schedule(self) {
        self.convert_to_schedule()
            .schedule()
    }
}
