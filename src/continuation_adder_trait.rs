use EndScheduleTrait;
use LooseContinuation;
use LooseContinuationIntoIterator;
use Task;
use TaskBox;
use TaskBoxIntoIterator;

pub trait ContinuationAdderTrait<TContinuationAdder: ContinuationAdderTrait<TContinuationAdder>>: EndScheduleTrait {
    fn add_continuation<TTask: 'static + Task>(self, continuation: TTask) -> TContinuationAdder;

    fn add_continuation_box(self, continuation_box: TaskBox) -> TContinuationAdder;

    fn add_continuation_boxes<TTaskBoxIntoIterator: 'static + TaskBoxIntoIterator>(self, continuation_boxes: TTaskBoxIntoIterator) -> TContinuationAdder;

    fn add_loose_continuation(self, loose_continuation: LooseContinuation) -> TContinuationAdder;

    fn add_loose_continuations<TLooseContinuationIntoIterator: 'static + LooseContinuationIntoIterator>(self, loose_continuations: TLooseContinuationIntoIterator) -> TContinuationAdder;
}
