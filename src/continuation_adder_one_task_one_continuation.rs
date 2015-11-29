use ContinuationAdderOneTaskMultipleContinuations;
use ContinuationAdderTrait;
use LooseContinuation;
use LooseContinuationIntoIterator;
use ScheduleOneTaskOneContinuation;
use ScheduleTrait;
use Task;
use TaskBox;
use TaskBoxIntoIterator;

pub struct ContinuationAdderOneTaskOneContinuation;

impl ContinuationAdderOneTaskOneContinuation {
    fn convert_to_continuation_adder_one_task_multiple_continuations(self) -> ContinuationAdderOneTaskMultipleContinuations {
        ContinuationAdderOneTaskMultipleContinuations
    }

    fn convert_to_schedule(self) -> ScheduleOneTaskOneContinuation {
        ScheduleOneTaskOneContinuation
    }
}

impl ContinuationAdderTrait<ContinuationAdderOneTaskMultipleContinuations,
                            ContinuationAdderOneTaskMultipleContinuations> for ContinuationAdderOneTaskOneContinuation {
    fn add_continuation<TTask: 'static + Task>(self, continuation: TTask) -> ContinuationAdderOneTaskMultipleContinuations {
        self.convert_to_continuation_adder_one_task_multiple_continuations()
            .add_continuation(continuation)
    }

    fn add_continuation_box(self, continuation_box: TaskBox) -> ContinuationAdderOneTaskMultipleContinuations {
        self.convert_to_continuation_adder_one_task_multiple_continuations()
            .add_continuation_box(continuation_box)
    }

    fn add_continuation_boxes<TTaskBoxIntoIterator: 'static + TaskBoxIntoIterator>(self, continuation_boxes: TTaskBoxIntoIterator) -> ContinuationAdderOneTaskMultipleContinuations {
        self.convert_to_continuation_adder_one_task_multiple_continuations()
            .add_continuation_boxes(continuation_boxes)
    }

    fn add_loose_continuation(self, loose_continuation: LooseContinuation) -> ContinuationAdderOneTaskMultipleContinuations {
        self.convert_to_continuation_adder_one_task_multiple_continuations()
            .add_loose_continuation(loose_continuation)
    }

    fn add_loose_continuations<TLooseContinuationIntoIterator: 'static + LooseContinuationIntoIterator>(self, loose_continuations: TLooseContinuationIntoIterator) -> ContinuationAdderOneTaskMultipleContinuations {
        self.convert_to_continuation_adder_one_task_multiple_continuations()
            .add_loose_continuations(loose_continuations)
    }
}

impl ScheduleTrait for ContinuationAdderOneTaskOneContinuation {
    fn schedule(self) {
        self.convert_to_schedule()
            .schedule()
    }
}
