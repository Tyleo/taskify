use ContinuationAdderTrait;
use ScheduleTrait;
use Task;
use TaskBox;
use TaskBoxIntoIterator;

pub trait TaskAdderTrait<TContinuationAdderMultipleContinuations,
                         TContinuationAdderOneContinuation,
                         TTaskAdderMultipleTasks,
                         TTaskAdderOneTask>: ContinuationAdderTrait<TContinuationAdderMultipleContinuations, TContinuationAdderOneContinuation> + ScheduleTrait
        where TContinuationAdderMultipleContinuations: ContinuationAdderTrait<TContinuationAdderMultipleContinuations, TContinuationAdderOneContinuation>,
              TContinuationAdderOneContinuation: ContinuationAdderTrait<TContinuationAdderOneContinuation, TContinuationAdderOneContinuation>,
              TTaskAdderMultipleTasks: TaskAdderTrait<TContinuationAdderMultipleContinuations, TContinuationAdderOneContinuation, TTaskAdderMultipleTasks, TTaskAdderOneTask>,
              TTaskAdderOneTask: TaskAdderTrait<TContinuationAdderMultipleContinuations, TContinuationAdderOneContinuation, TTaskAdderMultipleTasks, TTaskAdderOneTask> {
    fn add_task<TTask: 'static + Task>(self, task: TTask) -> TTaskAdderOneTask;

    fn add_task_box(self, task_box: TaskBox) -> TTaskAdderOneTask;

    fn add_task_boxes<TTaskBoxIntoIterator: 'static + TaskBoxIntoIterator>(self, task_boxes: TTaskBoxIntoIterator) -> TTaskAdderMultipleTasks;
}
