use ContinuationAdderTrait;
use LooseContinuation;
use LooseContinuationIntoIterator;
use ScheduleMultipleTasksMultipleContinuations;
use ScheduleTrait;
use Task;
use TaskBox;
use TaskBoxIntoIterator;

pub struct ContinuationAdderMultipleTasksMultipleContinuations;

impl ContinuationAdderMultipleTasksMultipleContinuations {
    fn convert_to_schedule(self) -> ScheduleMultipleTasksMultipleContinuations {
        ScheduleMultipleTasksMultipleContinuations
    }
}

impl ContinuationAdderTrait<ContinuationAdderMultipleTasksMultipleContinuations,
                            ContinuationAdderMultipleTasksMultipleContinuations> for ContinuationAdderMultipleTasksMultipleContinuations {
    fn add_continuation<TTask: 'static + Task>(self, continuation: TTask) -> ContinuationAdderMultipleTasksMultipleContinuations {
        self
    }

    fn add_continuation_box(self, continuation_box: TaskBox) -> ContinuationAdderMultipleTasksMultipleContinuations {
        self
    }

    fn add_continuation_boxes<TTaskBoxIntoIterator: 'static + TaskBoxIntoIterator>(self, continuation_boxes: TTaskBoxIntoIterator) -> ContinuationAdderMultipleTasksMultipleContinuations {
        self
    }

    fn add_loose_continuation(self, loose_continuation: LooseContinuation) -> ContinuationAdderMultipleTasksMultipleContinuations {
        self
    }

    fn add_loose_continuations<TLooseContinuationIntoIterator: 'static + LooseContinuationIntoIterator>(self, loose_continuations: TLooseContinuationIntoIterator) -> ContinuationAdderMultipleTasksMultipleContinuations {
        self
    }
}

impl ScheduleTrait for ContinuationAdderMultipleTasksMultipleContinuations {
    fn schedule(self) {
        self.convert_to_schedule()
            .schedule()
    }
}
