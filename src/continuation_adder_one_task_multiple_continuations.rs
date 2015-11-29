use ContinuationAdderTrait;
use LooseContinuation;
use LooseContinuationIntoIterator;
use ScheduleOneTaskMultipleContinuations;
use ScheduleTrait;
use Task;
use TaskBox;
use TaskBoxIntoIterator;

pub struct ContinuationAdderOneTaskMultipleContinuations;

impl ContinuationAdderOneTaskMultipleContinuations {
    fn convert_to_schedule(self) -> ScheduleOneTaskMultipleContinuations {
        ScheduleOneTaskMultipleContinuations
    }
}

impl ContinuationAdderTrait<ContinuationAdderOneTaskMultipleContinuations,
                            ContinuationAdderOneTaskMultipleContinuations> for ContinuationAdderOneTaskMultipleContinuations {
    fn add_continuation<TTask: 'static + Task>(self, continuation: TTask) -> ContinuationAdderOneTaskMultipleContinuations {
        self
    }

    fn add_continuation_box(self, continuation_box: TaskBox) -> ContinuationAdderOneTaskMultipleContinuations {
        self
    }

    fn add_continuation_boxes<TTaskBoxIntoIterator: 'static + TaskBoxIntoIterator>(self, continuation_boxes: TTaskBoxIntoIterator) -> ContinuationAdderOneTaskMultipleContinuations {
        self
    }

    fn add_loose_continuation(self, loose_continuation: LooseContinuation) -> ContinuationAdderOneTaskMultipleContinuations {
        self
    }

    fn add_loose_continuations<TLooseContinuationIntoIterator: 'static + LooseContinuationIntoIterator>(self, loose_continuations: TLooseContinuationIntoIterator) -> ContinuationAdderOneTaskMultipleContinuations {
        self
    }
}

impl ScheduleTrait for ContinuationAdderOneTaskMultipleContinuations {
    fn schedule(self) {
        self.convert_to_schedule()
            .schedule()
    }
}
