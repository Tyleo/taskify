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
    fn add_task<TTask: 'static + Task>(self, task: TTask) -> TTaskAdderMultipleTasks;

    fn add_task_box(self, task_box: TaskBox) -> TTaskAdderMultipleTasks;

    fn add_task_boxes<TTaskBoxIntoIterator: 'static + TaskBoxIntoIterator>(self, task_boxes: TTaskBoxIntoIterator) -> TTaskAdderMultipleTasks;
}
