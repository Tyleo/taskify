use ContinuationAdderTrait;
use EndScheduleTrait;
use SchedulerTrait;
use Task;
use TaskBox;
use TaskBoxIntoIterator;

pub trait TaskAdderTrait<TScheduler,
                         TContinuationAdderMultipleTaskBoxesMultipleContinuationBoxes,
                         TContinuationAdderMultipleTaskBoxesOneContinuationBox,
                         TContinuationAdderOneTaskBoxMultipleContinuationBoxes,
                         TContinuationAdderOneTaskBoxOneContinuationBox,
                         TTaskAdderMultipleTaskBoxes>: ContinuationAdderTrait<TScheduler,
                                                                              TContinuationAdderOneTaskBoxMultipleContinuationBoxes,
                                                                              TContinuationAdderOneTaskBoxOneContinuationBox> +
                                                       EndScheduleTrait
    where TScheduler: SchedulerTrait,
          TContinuationAdderMultipleTaskBoxesMultipleContinuationBoxes: ContinuationAdderTrait<TScheduler,
                                                                                               TContinuationAdderMultipleTaskBoxesMultipleContinuationBoxes,
                                                                                               TContinuationAdderMultipleTaskBoxesMultipleContinuationBoxes>,
          TContinuationAdderMultipleTaskBoxesOneContinuationBox: ContinuationAdderTrait<TScheduler,
                                                                                        TContinuationAdderMultipleTaskBoxesMultipleContinuationBoxes,
                                                                                        TContinuationAdderMultipleTaskBoxesMultipleContinuationBoxes>,
          TContinuationAdderOneTaskBoxMultipleContinuationBoxes: ContinuationAdderTrait<TScheduler,
                                                                                        TContinuationAdderOneTaskBoxMultipleContinuationBoxes,
                                                                                        TContinuationAdderOneTaskBoxMultipleContinuationBoxes>,
          TContinuationAdderOneTaskBoxOneContinuationBox: ContinuationAdderTrait<TScheduler,
                                                                                 TContinuationAdderOneTaskBoxMultipleContinuationBoxes,
                                                                                 TContinuationAdderOneTaskBoxMultipleContinuationBoxes>,
          TTaskAdderMultipleTaskBoxes: TaskAdderTrait<TScheduler,
                                                      TContinuationAdderMultipleTaskBoxesMultipleContinuationBoxes,
                                                      TContinuationAdderMultipleTaskBoxesOneContinuationBox,
                                                      TContinuationAdderMultipleTaskBoxesMultipleContinuationBoxes,
                                                      TContinuationAdderMultipleTaskBoxesOneContinuationBox,
                                                      TTaskAdderMultipleTaskBoxes> {
    fn add_task<TTask>(self,
                       task: TTask) -> TTaskAdderMultipleTaskBoxes
        where TTask: 'static +
                     Task<TScheduler::TTaskBoxParam>;

    fn add_task_box(self,
                    task_box: TaskBox<TScheduler::TTaskBoxParam>) -> TTaskAdderMultipleTaskBoxes;

    fn add_task_boxes<TTaskBoxIntoIterator>(self,
                                            task_boxes: TTaskBoxIntoIterator) -> TTaskAdderMultipleTaskBoxes
        where TTaskBoxIntoIterator: TaskBoxIntoIterator<TScheduler::TTaskBoxParam>;
}
