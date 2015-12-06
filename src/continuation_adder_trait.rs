use EndScheduleTrait;
use Task;
use TaskBox;
use TaskBoxIntoIterator;

pub trait ContinuationAdderTrait<TContinuationAdderMultipleContinuationBoxes,
                                 TContinuationAdderOneContinuationBox>: EndScheduleTrait<()>
        where TContinuationAdderMultipleContinuationBoxes: ContinuationAdderTrait<TContinuationAdderMultipleContinuationBoxes, TContinuationAdderMultipleContinuationBoxes>,
              TContinuationAdderOneContinuationBox: ContinuationAdderTrait<TContinuationAdderMultipleContinuationBoxes, TContinuationAdderMultipleContinuationBoxes> {
    fn add_continuation<TTask>(self,
                               continuation: TTask) -> TContinuationAdderOneContinuationBox
        where TTask: 'static +
                     Task;

    fn add_continuation_box(self,
                            continuation_box: TaskBox) -> TContinuationAdderOneContinuationBox;

    fn add_continuation_boxes<TTaskBoxIntoIterator>(self,
                                                    continuation_boxes: TTaskBoxIntoIterator) -> TContinuationAdderMultipleContinuationBoxes
        where TTaskBoxIntoIterator: 'static +
                                    TaskBoxIntoIterator;
}
