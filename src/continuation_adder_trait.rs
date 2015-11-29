use ScheduleTrait;
use LooseContinuation;
use LooseContinuationIntoIterator;
use Task;
use TaskBox;
use TaskBoxIntoIterator;

pub trait ContinuationAdderTrait<TContinuationAdderMultipleContinuations,
                                 TContinuationAdderOneContinuation>: ScheduleTrait
        where TContinuationAdderMultipleContinuations: ContinuationAdderTrait<TContinuationAdderMultipleContinuations, TContinuationAdderMultipleContinuations>,
              TContinuationAdderOneContinuation: ContinuationAdderTrait<TContinuationAdderMultipleContinuations, TContinuationAdderMultipleContinuations> {
    fn add_continuation<TTask: 'static + Task>(self, continuation: TTask) -> TContinuationAdderOneContinuation;

    fn add_continuation_box(self, continuation_box: TaskBox) -> TContinuationAdderOneContinuation;

    fn add_continuation_boxes<TTaskBoxIntoIterator: 'static + TaskBoxIntoIterator>(self, continuation_boxes: TTaskBoxIntoIterator) -> TContinuationAdderMultipleContinuations;

    fn add_loose_continuation(self, loose_continuation: LooseContinuation) -> TContinuationAdderOneContinuation;

    fn add_loose_continuations<TLooseContinuationIntoIterator: 'static + LooseContinuationIntoIterator>(self, loose_continuations: TLooseContinuationIntoIterator) -> TContinuationAdderMultipleContinuations;
}
