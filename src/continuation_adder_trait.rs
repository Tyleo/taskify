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
    fn add_continuation<TTask>(self,
                               continuation: TTask) -> TContinuationAdderOneContinuation
        where TTask: 'static +
                     Task;

    fn add_continuation_box(self,
                            continuation_box: TaskBox) -> TContinuationAdderOneContinuation;

    fn add_continuation_boxes<TTaskBoxIntoIterator>(self,
                                                    continuation_boxes: TTaskBoxIntoIterator) -> TContinuationAdderMultipleContinuations
        where TTaskBoxIntoIterator: 'static +
                                    TaskBoxIntoIterator;

    // fn add_loose_continuation(self, loose_continuation: LooseContinuation) -> TContinuationAdderOneContinuation;

    // fn add_loose_continuations<TLooseContinuationIntoIterator: 'static + LooseContinuationIntoIterator>(self, loose_continuations: TLooseContinuationIntoIterator) -> TContinuationAdderMultipleContinuations;
}
