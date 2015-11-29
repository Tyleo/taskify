use BeginScheduleTrait;
use ContinuationAdderMultipleTasksMultipleContinuations;
use ContinuationAdderMultipleTasksOneContinuation;
use ContinuationAdderOneTaskMultipleContinuations;
use ContinuationAdderOneTaskOneContinuation;
use EmptyTaskAdder;
use TaskAdderMultipleTasks;
use TaskAdderOneTask;

pub struct Scheduler;

impl Scheduler {
    fn convert_to_empty_task_adder<'a>(&'a self) -> EmptyTaskAdder<'a> {
        EmptyTaskAdder::new(self)
    }
}

impl <'a> BeginScheduleTrait<'a,
                             ContinuationAdderMultipleTasksMultipleContinuations<'a>,
                             ContinuationAdderMultipleTasksOneContinuation<'a>,
                             ContinuationAdderOneTaskMultipleContinuations<'a>,
                             ContinuationAdderOneTaskOneContinuation<'a>,
                             EmptyTaskAdder<'a>,
                             TaskAdderMultipleTasks<'a>,
                             TaskAdderOneTask<'a>> for Scheduler {
    fn begin_schedule(&'a self) -> EmptyTaskAdder<'a> {
        self.convert_to_empty_task_adder()
    }
}
