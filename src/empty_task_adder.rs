use ContinuationAdderMultipleTasksMultipleContinuations;
use ContinuationAdderMultipleTasksOneContinuation;
use ContinuationAdderOneTaskMultipleContinuations;
use ContinuationAdderOneTaskOneContinuation;
use EmptyTaskAdderTrait;
use Scheduler;
use Task;
use TaskAdderMultipleTasks;
use TaskAdderOneTask;
use TaskAdderTrait;
use TaskBox;
use TaskBoxIntoIterator;

pub struct EmptyTaskAdder<'a> {
    scheduler: &'a Scheduler,
}

impl <'a> EmptyTaskAdder<'a> {
    pub fn new(scheduler: &'a Scheduler) -> EmptyTaskAdder<'a> {
        EmptyTaskAdder { scheduler: scheduler }
    }

    fn convert_to_task_adder_multiple_task_boxes(self) -> TaskAdderMultipleTasks<'a> {
        TaskAdderMultipleTasks::new(self.scheduler,
                                    Vec::new())
    }

    fn convert_to_task_adder_one_task_box(self,
                                          task_box: TaskBox) -> TaskAdderOneTask<'a> {
        TaskAdderOneTask::new(self.scheduler,
                              task_box)
    }
}

impl <'a> EmptyTaskAdderTrait<ContinuationAdderMultipleTasksMultipleContinuations<'a>,
                              ContinuationAdderMultipleTasksOneContinuation<'a>,
                              ContinuationAdderOneTaskMultipleContinuations<'a>,
                              ContinuationAdderOneTaskOneContinuation<'a>,
                              TaskAdderMultipleTasks<'a>,
                              TaskAdderOneTask<'a>> for EmptyTaskAdder<'a> {
    fn add_task<TTask>(self,
                       task: TTask) -> TaskAdderOneTask<'a>
        where TTask: 'static +
                     Task {
        let task_box = Box::new(task);
        self.add_task_box(task_box)
    }

    fn add_task_box(self,
                    task_box: TaskBox) -> TaskAdderOneTask<'a> {
        self.convert_to_task_adder_one_task_box(task_box)
    }

    fn add_task_boxes<TTaskBoxIntoIterator>(self,
                                            task_boxes: TTaskBoxIntoIterator) -> TaskAdderMultipleTasks<'a>
        where TTaskBoxIntoIterator: 'static +
                                    TaskBoxIntoIterator {
        self.convert_to_task_adder_multiple_task_boxes()
            .add_task_boxes(task_boxes)
    }
}
