use fluent::EndScheduleTrait;
use SchedulerTrait;
use Task;
use TaskBox;
use TaskBoxIntoIterator;

pub trait ContinuationAdderTrait<TScheduler,
                                 TContinuationAdderMultipleContinuationBoxes,
                                 TContinuationAdderOneContinuationBox>: EndScheduleTrait
        where TScheduler: SchedulerTrait,
              TContinuationAdderMultipleContinuationBoxes: ContinuationAdderTrait<TScheduler,
                                                                                  TContinuationAdderMultipleContinuationBoxes,
                                                                                  TContinuationAdderMultipleContinuationBoxes>,
              TContinuationAdderOneContinuationBox: ContinuationAdderTrait<TScheduler,
                                                                           TContinuationAdderMultipleContinuationBoxes,
                                                                           TContinuationAdderMultipleContinuationBoxes> {
    fn add_continuation<TTask>(self,
                               continuation: TTask) -> TContinuationAdderOneContinuationBox
        where TTask: 'static +
                     Task<TScheduler::TTaskBoxParam>;

    fn add_continuation_box(self,
                            continuation_box: TaskBox<TScheduler::TTaskBoxParam>) -> TContinuationAdderOneContinuationBox;

    fn add_continuation_boxes<TTaskBoxIntoIterator>(self,
                                                    continuation_boxes: TTaskBoxIntoIterator) -> TContinuationAdderMultipleContinuationBoxes
        where TTaskBoxIntoIterator: TaskBoxIntoIterator<TScheduler::TTaskBoxParam>;
}
