use ContinuationAdderMultipleTasksMultipleContinuations;
use ContinuationAdderMultipleTasksOneContinuation;
use ContinuationAdderOneTaskMultipleContinuations;
use ContinuationAdderOneTaskOneContinuation;
use Task;
use TaskAdderMultipleTasks;
use TaskAdderHasNoTasksTrait;
use TaskAdderHasTasksTrait;
use TaskAdderOneTask;
use TaskBox;
use TaskBoxIntoIterator;

pub struct Scheduler;

impl Scheduler {
    fn convert_to_task_adder_multiple_tasks<'a>(&'a self) -> TaskAdderMultipleTasks<'a> {
        TaskAdderMultipleTasks::new(self)
    }

    fn convert_to_task_adder_one_task<'a>(&'a self) -> TaskAdderOneTask<'a> {
        TaskAdderOneTask::new(self)
    }
}

impl <'a> TaskAdderHasNoTasksTrait<'a,
                                   ContinuationAdderMultipleTasksMultipleContinuations<'a>,
                                   ContinuationAdderMultipleTasksOneContinuation<'a>,
                                   ContinuationAdderOneTaskMultipleContinuations<'a>,
                                   ContinuationAdderOneTaskOneContinuation<'a>,
                                   TaskAdderMultipleTasks<'a>,
                                   TaskAdderOneTask<'a>> for Scheduler {
    fn add_task<TTask: 'static + Task>(&'a self, task: TTask) -> TaskAdderOneTask<'a> {
        self.convert_to_task_adder_one_task()
    }

    fn add_task_box(&'a self, task_box: TaskBox) -> TaskAdderOneTask<'a> {
        self.convert_to_task_adder_one_task()
    }

    fn add_task_boxes<TTaskBoxIntoIterator: 'static + TaskBoxIntoIterator>(&'a self, task_boxes: TTaskBoxIntoIterator) -> TaskAdderMultipleTasks<'a> {
        self.convert_to_task_adder_multiple_tasks()
            .add_task_boxes(task_boxes)
    }
}
