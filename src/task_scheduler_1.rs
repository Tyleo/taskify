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

pub struct TaskScheduler1;

impl TaskScheduler1 {
    fn convert_to_task_adder_multiple_tasks(&self) -> TaskAdderMultipleTasks {
        TaskAdderMultipleTasks
    }

    fn convert_to_task_adder_one_task(&self) -> TaskAdderOneTask {
        TaskAdderOneTask
    }
}

impl TaskAdderHasNoTasksTrait<ContinuationAdderMultipleTasksMultipleContinuations,
                              ContinuationAdderMultipleTasksOneContinuation,
                              ContinuationAdderOneTaskMultipleContinuations,
                              ContinuationAdderOneTaskOneContinuation,
                              TaskAdderMultipleTasks,
                              TaskAdderOneTask> for TaskScheduler1 {
    fn add_task<TTask: 'static + Task>(&self, task: TTask) -> TaskAdderOneTask {
        self.convert_to_task_adder_one_task()
    }

    fn add_task_box(&self, task_box: TaskBox) -> TaskAdderOneTask {
        self.convert_to_task_adder_one_task()
    }

    fn add_task_boxes<TTaskBoxIntoIterator: 'static + TaskBoxIntoIterator>(&self, task_boxes: TTaskBoxIntoIterator) -> TaskAdderMultipleTasks {
        self.convert_to_task_adder_multiple_tasks()
            .add_task_boxes(task_boxes)
    }
}
