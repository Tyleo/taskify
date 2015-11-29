use ContinuationAdderTrait;
use EndSchedule;
use EndScheduleTrait;
use LooseContinuation;
use LooseContinuationIntoIterator;
use Task;
use TaskBox;
use TaskBoxIntoIterator;

pub struct ContinuationAdder;

impl ContinuationAdder {
    fn ConvertToEndSchedule(self) -> EndSchedule {
        EndSchedule
    }
}

impl ContinuationAdderTrait<ContinuationAdder> for ContinuationAdder {
    fn add_continuation<TTask: 'static + Task>(self, continuation: TTask) -> ContinuationAdder {
        self
    }

    fn add_continuation_box(self, continuation_box: TaskBox) -> ContinuationAdder {
        self
    }

    fn add_continuation_boxes<TTaskBoxIntoIterator: 'static + TaskBoxIntoIterator>(self, continuation_boxes: TTaskBoxIntoIterator) -> ContinuationAdder {
        self
    }

    fn add_loose_continuation(self, loose_continuation: LooseContinuation) -> ContinuationAdder {
        self
    }

    fn add_loose_continuations<TLooseContinuationIntoIterator: 'static + LooseContinuationIntoIterator>(self, loose_continuations: TLooseContinuationIntoIterator) -> ContinuationAdder {
        self
    }
}

impl EndScheduleTrait for ContinuationAdder {
    fn EndSchedule(self) {
        self.ConvertToEndSchedule()
            .EndSchedule();
    }
}
