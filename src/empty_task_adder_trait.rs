use ContinuationAdderTrait;
use Task;
use TaskAdderTrait;
use TaskBox;
use TaskBoxIntoIterator;

pub trait EmptyTaskAdderTrait<TContinuationAdderMultipleTaskBoxesMultipleContinuationBoxes,
                              TContinuationAdderMultipleTaskBoxesOneContinuationBox,
                              TContinuationAdderOneTaskBoxMultipleContinuationBoxes,
                              TContinuationAdderOneTaskBoxOneContinuationBox,
                              TTaskAdderMultipleTaskBoxes,
                              TTaskAdderOneTaskBox>
        where TContinuationAdderMultipleTaskBoxesMultipleContinuationBoxes: ContinuationAdderTrait<TContinuationAdderMultipleTaskBoxesMultipleContinuationBoxes,
                                                                                                   TContinuationAdderMultipleTaskBoxesMultipleContinuationBoxes>,
              TContinuationAdderMultipleTaskBoxesOneContinuationBox: ContinuationAdderTrait<TContinuationAdderMultipleTaskBoxesMultipleContinuationBoxes,
                                                                                            TContinuationAdderMultipleTaskBoxesMultipleContinuationBoxes>,
              TContinuationAdderOneTaskBoxMultipleContinuationBoxes: ContinuationAdderTrait<TContinuationAdderOneTaskBoxMultipleContinuationBoxes,
                                                                                            TContinuationAdderOneTaskBoxMultipleContinuationBoxes>,
              TContinuationAdderOneTaskBoxOneContinuationBox: ContinuationAdderTrait<TContinuationAdderOneTaskBoxMultipleContinuationBoxes,
                                                                                     TContinuationAdderOneTaskBoxMultipleContinuationBoxes>,
              TTaskAdderMultipleTaskBoxes: TaskAdderTrait<TContinuationAdderMultipleTaskBoxesMultipleContinuationBoxes,
                                                          TContinuationAdderMultipleTaskBoxesOneContinuationBox,
                                                          TContinuationAdderMultipleTaskBoxesMultipleContinuationBoxes,
                                                          TContinuationAdderMultipleTaskBoxesOneContinuationBox,
                                                          TTaskAdderMultipleTaskBoxes>,
              TTaskAdderOneTaskBox: TaskAdderTrait<TContinuationAdderMultipleTaskBoxesMultipleContinuationBoxes,
                                                   TContinuationAdderMultipleTaskBoxesOneContinuationBox,
                                                   TContinuationAdderOneTaskBoxMultipleContinuationBoxes,
                                                   TContinuationAdderOneTaskBoxOneContinuationBox,
                                                   TTaskAdderMultipleTaskBoxes> {
    fn add_task<TTask>(self,
                       task: TTask) -> TTaskAdderOneTaskBox
        where TTask: 'static +
                     Task;

    fn add_task_box(self,
                    task_box: TaskBox) -> TTaskAdderOneTaskBox;

    fn add_task_boxes<TTaskBoxIntoIterator>(self,
                                            task_boxes: TTaskBoxIntoIterator) -> TTaskAdderMultipleTaskBoxes
        where TTaskBoxIntoIterator: TaskBoxIntoIterator;
}
