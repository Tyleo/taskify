use fluent::ContinuationAdderTrait;
use SchedulerTrait;
use Task;
use fluent::TaskAdderTrait;
use TaskBox;
use TaskBoxIntoIterator;

pub trait EmptyTaskAdderTrait<TScheduler,
                              TContinuationAdderMultipleTaskBoxesMultipleContinuationBoxes,
                              TContinuationAdderMultipleTaskBoxesOneContinuationBox,
                              TContinuationAdderOneTaskBoxMultipleContinuationBoxes,
                              TContinuationAdderOneTaskBoxOneContinuationBox,
                              TTaskAdderMultipleTaskBoxes,
                              TTaskAdderOneTaskBox>
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
                                                          TTaskAdderMultipleTaskBoxes>,
              TTaskAdderOneTaskBox: TaskAdderTrait<TScheduler,
                                                   TContinuationAdderMultipleTaskBoxesMultipleContinuationBoxes,
                                                   TContinuationAdderMultipleTaskBoxesOneContinuationBox,
                                                   TContinuationAdderOneTaskBoxMultipleContinuationBoxes,
                                                   TContinuationAdderOneTaskBoxOneContinuationBox,
                                                   TTaskAdderMultipleTaskBoxes> {
    fn add_task<TTask>(self,
                       task: TTask) -> TTaskAdderOneTaskBox
        where TTask: 'static +
                     Task<TScheduler::TTaskBoxParam>;

    fn add_task_box(self,
                    task_box: TaskBox<TScheduler::TTaskBoxParam>) -> TTaskAdderOneTaskBox;

    fn add_task_boxes<TTaskBoxIntoIterator>(self,
                                            task_boxes: TTaskBoxIntoIterator) -> TTaskAdderMultipleTaskBoxes
        where TTaskBoxIntoIterator: TaskBoxIntoIterator<TScheduler::TTaskBoxParam>;
}
