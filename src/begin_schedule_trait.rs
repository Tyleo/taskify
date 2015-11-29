use ContinuationAdderTrait;
use EmptyTaskAdderTrait;
use TaskAdderTrait;

pub trait BeginScheduleTrait<'a,
                             TContinuationAdderMultipleTasksMultipleContinuations,
                             TContinuationAdderMultipleTasksOneContinuation,
                             TContinuationAdderOneTaskMultipleContinuations,
                             TContinuationAdderOneTaskOneContinuation,
                             TEmptyTaskAdder,
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
          TEmptyTaskAdder: EmptyTaskAdderTrait<TContinuationAdderMultipleTasksMultipleContinuations,
                                               TContinuationAdderMultipleTasksOneContinuation,
                                               TContinuationAdderOneTaskMultipleContinuations,
                                               TContinuationAdderOneTaskOneContinuation,
                                               TTaskAdderMultipleTasks,
                                               TTaskAdderOneTask>,
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
    fn begin_schedule(&'a self) -> TEmptyTaskAdder;
}
