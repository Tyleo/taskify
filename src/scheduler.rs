use ContinuationAdderMultipleTasksMultipleContinuations;
use ContinuationAdderMultipleTasksOneContinuation;
use ContinuationAdderOneTaskMultipleContinuations;
use ContinuationAdderOneTaskOneContinuation;
use Task;
use TaskAdderMultipleTasks;
use TaskAdderHasNoTasksTrait;
use TaskAdderTrait;
use TaskAdderOneTask;
use TaskBox;
use TaskBoxIntoIterator;

pub struct Scheduler;

impl Scheduler {
    fn convert_to_task_adder_multiple_tasks<'a>(&'a self) -> TaskAdderMultipleTasks<'a> {
        TaskAdderMultipleTasks::new(self,
                                    Vec::new())
    }

    fn convert_to_task_adder_one_task<'a>(&'a self,
                                          task_box: TaskBox) -> TaskAdderOneTask<'a> {
        TaskAdderOneTask::new(self,
                              task_box)
    }
}

impl <'a> TaskAdderHasNoTasksTrait<'a,
                                   ContinuationAdderMultipleTasksMultipleContinuations<'a>,
                                   ContinuationAdderMultipleTasksOneContinuation<'a>,
                                   ContinuationAdderOneTaskMultipleContinuations<'a>,
                                   ContinuationAdderOneTaskOneContinuation<'a>,
                                   TaskAdderMultipleTasks<'a>,
                                   TaskAdderOneTask<'a>> for Scheduler {
    fn add_task<TTask>(&'a self,
                       task: TTask) -> TaskAdderOneTask<'a>
        where TTask: 'static +
                     Task {
        let task_box = Box::new(task);
        self.add_task_box(task_box)
    }

    fn add_task_box(&'a self,
                    task_box: TaskBox) -> TaskAdderOneTask<'a> {
        self.convert_to_task_adder_one_task(task_box)
    }

    fn add_task_boxes<TTaskBoxIntoIterator>(&'a self,
                                            task_boxes: TTaskBoxIntoIterator) -> TaskAdderMultipleTasks<'a>
        where TTaskBoxIntoIterator: 'static +
                                    TaskBoxIntoIterator {
        self.convert_to_task_adder_multiple_tasks()
            .add_task_boxes(task_boxes)
    }
}
