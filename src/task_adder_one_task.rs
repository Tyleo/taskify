use ContinuationAdderMultipleTasksMultipleContinuations;
use ContinuationAdderMultipleTasksOneContinuation;
use ContinuationAdderOneTaskMultipleContinuations;
use ContinuationAdderOneTaskOneContinuation;
use ContinuationAdderTrait;
use ScheduleOneTaskNoContinuations;
use ScheduleTrait;
use LooseContinuation;
use LooseContinuationIntoIterator;
use Task;
use TaskAdderHasTasksTrait;
use TaskAdderMultipleTasks;
use TaskBox;
use TaskBoxIntoIterator;

pub struct TaskAdderOneTask;

impl TaskAdderOneTask {
    fn convert_to_task_adder_multiple_tasks(self) -> TaskAdderMultipleTasks {
        TaskAdderMultipleTasks
    }

    fn convert_to_continuation_adder_one_task_multiple_continuations(self) -> ContinuationAdderOneTaskMultipleContinuations {
        ContinuationAdderOneTaskMultipleContinuations
    }

    fn convert_to_continuation_adder_one_task_one_continuation(self) -> ContinuationAdderOneTaskOneContinuation {
        ContinuationAdderOneTaskOneContinuation
    }

    fn convert_to_schedule_one_task_no_continuations(self) -> ScheduleOneTaskNoContinuations {
        ScheduleOneTaskNoContinuations
    }
}

impl TaskAdderHasTasksTrait<ContinuationAdderMultipleTasksMultipleContinuations,
                            ContinuationAdderMultipleTasksOneContinuation,
                            ContinuationAdderOneTaskMultipleContinuations,
                            ContinuationAdderOneTaskOneContinuation,
                            TaskAdderMultipleTasks> for TaskAdderOneTask {
    fn add_task<TTask: 'static + Task>(self, task: TTask) -> TaskAdderMultipleTasks {
        self.convert_to_task_adder_multiple_tasks()
            .add_task(task)
    }

    fn add_task_box(self, task_box: TaskBox) -> TaskAdderMultipleTasks {
        self.convert_to_task_adder_multiple_tasks()
            .add_task_box(task_box)
    }

    fn add_task_boxes<TTaskBoxIntoIterator: 'static + TaskBoxIntoIterator>(self, task_boxes: TTaskBoxIntoIterator) -> TaskAdderMultipleTasks {
        self.convert_to_task_adder_multiple_tasks()
            .add_task_boxes(task_boxes)
    }
}

impl ContinuationAdderTrait<ContinuationAdderOneTaskMultipleContinuations,
                            ContinuationAdderOneTaskOneContinuation> for TaskAdderOneTask {
    fn add_continuation<TTask: 'static + Task>(self, continuation: TTask) -> ContinuationAdderOneTaskOneContinuation {
        self.convert_to_continuation_adder_one_task_one_continuation()
    }

    fn add_continuation_box(self, continuation_box: TaskBox) -> ContinuationAdderOneTaskOneContinuation {
        self.convert_to_continuation_adder_one_task_one_continuation()
    }

    fn add_continuation_boxes<TTaskBoxIntoIterator: 'static + TaskBoxIntoIterator>(self, continuation_boxes: TTaskBoxIntoIterator) -> ContinuationAdderOneTaskMultipleContinuations {
        self.convert_to_continuation_adder_one_task_multiple_continuations()
            .add_continuation_boxes(continuation_boxes)
    }

    fn add_loose_continuation(self, loose_continuation: LooseContinuation) -> ContinuationAdderOneTaskOneContinuation {
        self.convert_to_continuation_adder_one_task_one_continuation()
    }

    fn add_loose_continuations<TLooseContinuationIntoIterator: 'static + LooseContinuationIntoIterator>(self, loose_continuations: TLooseContinuationIntoIterator) -> ContinuationAdderOneTaskMultipleContinuations {
        self.convert_to_continuation_adder_one_task_multiple_continuations()
            .add_loose_continuations(loose_continuations)
    }
}

impl ScheduleTrait for TaskAdderOneTask {
    fn schedule(self) {
        self.convert_to_schedule_one_task_no_continuations()
            .schedule();
    }
}
