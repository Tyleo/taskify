use ContinuationAdderTrait;
use Task;
use TaskAdderTrait;
use TaskBox;
use TaskBoxIntoIterator;

pub trait EmptyTaskAdderTrait<TContinuationAdderMultipleTasksMultipleContinuations,
                              TContinuationAdderMultipleTasksOneContinuation,
                              TContinuationAdderOneTaskMultipleContinuations,
                              TContinuationAdderOneTaskOneContinuation,
                              TTaskAdderMultipleTasks,
                              TTaskAdderOneTask>
        where TContinuationAdderMultipleTasksMultipleContinuations: ContinuationAdderTrait<TContinuationAdderMultipleTasksMultipleContinuations,
                                                                                           TContinuationAdderMultipleTasksMultipleContinuations>,
              TContinuationAdderMultipleTasksOneContinuation: ContinuationAdderTrait<TContinuationAdderMultipleTasksMultipleContinuations,
                                                                                     TContinuationAdderMultipleTasksMultipleContinuations>,
              TContinuationAdderOneTaskMultipleContinuations: ContinuationAdderTrait<TContinuationAdderOneTaskMultipleContinuations,
                                                                                     TContinuationAdderOneTaskMultipleContinuations>,
              TContinuationAdderOneTaskOneContinuation: ContinuationAdderTrait<TContinuationAdderOneTaskMultipleContinuations,
                                                                               TContinuationAdderOneTaskMultipleContinuations>,
              TTaskAdderMultipleTasks: TaskAdderTrait<TContinuationAdderMultipleTasksMultipleContinuations,
                                                      TContinuationAdderMultipleTasksOneContinuation,
                                                      TContinuationAdderMultipleTasksMultipleContinuations,
                                                      TContinuationAdderMultipleTasksOneContinuation,
                                                      TTaskAdderMultipleTasks>,
              TTaskAdderOneTask: TaskAdderTrait<TContinuationAdderMultipleTasksMultipleContinuations,
                                                TContinuationAdderMultipleTasksOneContinuation,
                                                TContinuationAdderOneTaskMultipleContinuations,
                                                TContinuationAdderOneTaskOneContinuation,
                                                TTaskAdderMultipleTasks> {
    fn add_task<TTask>(self,
                       task: TTask) -> TTaskAdderOneTask
        where TTask: 'static +
                     Task;

    fn add_task_box(self,
                    task_box: TaskBox) -> TTaskAdderOneTask;

    fn add_task_boxes<TTaskBoxIntoIterator>(self,
                                            task_boxes: TTaskBoxIntoIterator) -> TTaskAdderMultipleTasks
        where TTaskBoxIntoIterator: 'static +
                                    TaskBoxIntoIterator;
}
