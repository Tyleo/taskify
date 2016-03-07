use fluent::ContinuationAdderTrait;
use fluent::EmptyTaskAdderTrait;
use SchedulerTrait;
use fluent::TaskAdderTrait;

pub trait BeginScheduleTrait<'a,
                             TScheduler,
                             TContinuationAdderMultipleTasksMultipleContinuations,
                             TContinuationAdderMultipleTasksOneContinuation,
                             TContinuationAdderOneTaskMultipleContinuations,
                             TContinuationAdderOneTaskOneContinuation,
                             TEmptyTaskAdder,
                             TTaskAdderMultipleTasks,
                             TTaskAdderOneTask>
    where TScheduler: SchedulerTrait,
          TContinuationAdderMultipleTasksMultipleContinuations: ContinuationAdderTrait<TScheduler,
                                                                                       TContinuationAdderMultipleTasksMultipleContinuations,
                                                                                       TContinuationAdderMultipleTasksMultipleContinuations>,
          TContinuationAdderMultipleTasksOneContinuation: ContinuationAdderTrait<TScheduler,
                                                                                 TContinuationAdderMultipleTasksMultipleContinuations,
                                                                                 TContinuationAdderMultipleTasksMultipleContinuations>,
          TContinuationAdderOneTaskMultipleContinuations: ContinuationAdderTrait<TScheduler,
                                                                                 TContinuationAdderOneTaskMultipleContinuations,
                                                                                 TContinuationAdderOneTaskMultipleContinuations>,
          TContinuationAdderOneTaskOneContinuation: ContinuationAdderTrait<TScheduler,
                                                                           TContinuationAdderOneTaskMultipleContinuations,
                                                                           TContinuationAdderOneTaskMultipleContinuations>,
          TEmptyTaskAdder: EmptyTaskAdderTrait<TScheduler,
                                               TContinuationAdderMultipleTasksMultipleContinuations,
                                               TContinuationAdderMultipleTasksOneContinuation,
                                               TContinuationAdderOneTaskMultipleContinuations,
                                               TContinuationAdderOneTaskOneContinuation,
                                               TTaskAdderMultipleTasks,
                                               TTaskAdderOneTask>,
          TTaskAdderMultipleTasks: TaskAdderTrait<TScheduler,
                                                  TContinuationAdderMultipleTasksMultipleContinuations,
                                                  TContinuationAdderMultipleTasksOneContinuation,
                                                  TContinuationAdderMultipleTasksMultipleContinuations,
                                                  TContinuationAdderMultipleTasksOneContinuation,
                                                  TTaskAdderMultipleTasks>,
          TTaskAdderOneTask: TaskAdderTrait<TScheduler,
                                            TContinuationAdderMultipleTasksMultipleContinuations,
                                            TContinuationAdderMultipleTasksOneContinuation,
                                            TContinuationAdderOneTaskMultipleContinuations,
                                            TContinuationAdderOneTaskOneContinuation,
                                            TTaskAdderMultipleTasks> {
    fn begin_schedule(&'a self) -> TEmptyTaskAdder;
}
