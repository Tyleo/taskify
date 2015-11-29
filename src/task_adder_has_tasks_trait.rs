use ContinuationAdderTrait;
use ScheduleTrait;
use Task;
use TaskBox;
use TaskBoxIntoIterator;

pub trait TaskAdderHasTasksTrait<TContinuationAdderMultipleTasksMultipleContinuations,
                                 TContinuationAdderMultipleTasksOneContinuation,
                                 TContinuationAdderOneTaskMultipleContinuations,
                                 TContinuationAdderOneTaskOneContinuation,
                                 TTaskAdderMultipleTasks>: ContinuationAdderTrait<TContinuationAdderOneTaskMultipleContinuations,
                                                                                  TContinuationAdderOneTaskOneContinuation> + ScheduleTrait
        where TContinuationAdderMultipleTasksMultipleContinuations: ContinuationAdderTrait<TContinuationAdderMultipleTasksMultipleContinuations,
                                                                                           TContinuationAdderMultipleTasksMultipleContinuations>,
              TContinuationAdderMultipleTasksOneContinuation: ContinuationAdderTrait<TContinuationAdderMultipleTasksMultipleContinuations,
                                                                                     TContinuationAdderMultipleTasksMultipleContinuations>,
              TContinuationAdderOneTaskMultipleContinuations: ContinuationAdderTrait<TContinuationAdderOneTaskMultipleContinuations,
                                                                                     TContinuationAdderOneTaskMultipleContinuations>,
              TContinuationAdderOneTaskOneContinuation: ContinuationAdderTrait<TContinuationAdderOneTaskMultipleContinuations,
                                                                               TContinuationAdderOneTaskMultipleContinuations>,
              TTaskAdderMultipleTasks: TaskAdderHasTasksTrait<TContinuationAdderMultipleTasksMultipleContinuations,
                                                              TContinuationAdderMultipleTasksOneContinuation,
                                                              TContinuationAdderMultipleTasksMultipleContinuations,
                                                              TContinuationAdderMultipleTasksOneContinuation,
                                                              TTaskAdderMultipleTasks> {
    fn add_task<TTask>(self,
                       task: TTask) -> TTaskAdderMultipleTasks
        where TTask: 'static +
                     Task;

    fn add_task_box(self,
                    task_box: TaskBox) -> TTaskAdderMultipleTasks;

    fn add_task_boxes<TTaskBoxIntoIterator>(self,
                                            task_boxes: TTaskBoxIntoIterator) -> TTaskAdderMultipleTasks
        where TTaskBoxIntoIterator: 'static +
                                    TaskBoxIntoIterator;
}
