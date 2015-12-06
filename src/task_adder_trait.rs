use ContinuationAdderTrait;
use EndScheduleTrait;
use Task;
use TaskBox;
use TaskBoxIntoIterator;

pub trait TaskAdderTrait<TContinuationAdderMultipleTaskBoxesMultipleContinuationBoxes,
                         TContinuationAdderMultipleTaskBoxesOneContinuationBox,
                         TContinuationAdderOneTaskBoxMultipleContinuationBoxes,
                         TContinuationAdderOneTaskBoxOneContinuationBox,
                         TTaskAdderMultipleTaskBoxes>: ContinuationAdderTrait<TContinuationAdderOneTaskBoxMultipleContinuationBoxes,
                                                                              TContinuationAdderOneTaskBoxOneContinuationBox> +
                                                       EndScheduleTrait
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
                                                      TTaskAdderMultipleTaskBoxes> {
    fn add_task<TTask>(self,
                       task: TTask) -> TTaskAdderMultipleTaskBoxes
        where TTask: 'static +
                     Task;

    fn add_task_box(self,
                    task_box: TaskBox) -> TTaskAdderMultipleTaskBoxes;

    fn add_task_boxes<TTaskBoxIntoIterator>(self,
                                            task_boxes: TTaskBoxIntoIterator) -> TTaskAdderMultipleTaskBoxes
        where TTaskBoxIntoIterator: 'static +
                                    TaskBoxIntoIterator;
}
