use ContinuationAdderMultipleTasksMultipleContinuations;
use ContinuationAdderMultipleTasksOneContinuation;
use ContinuationAdderTrait;
use ScheduleMultipleTasksNoContinuations;
use ScheduleTrait;
use LooseContinuation;
use LooseContinuationIntoIterator;
use Task;
use TaskAdderHasTasksTrait;
use TaskBox;
use TaskBoxIntoIterator;

pub struct TaskAdderMultipleTasks;

impl TaskAdderMultipleTasks {
    fn convert_to_continuation_adder_multiple_tasks_multiple_continuations(self) -> ContinuationAdderMultipleTasksMultipleContinuations {
        ContinuationAdderMultipleTasksMultipleContinuations
    }

    fn convert_to_continuation_adder_multiple_tasks_one_continuation(self) -> ContinuationAdderMultipleTasksOneContinuation {
        ContinuationAdderMultipleTasksOneContinuation
    }

    fn convert_to_schedule_multiple_tasks_no_continuations(self) -> ScheduleMultipleTasksNoContinuations {
        ScheduleMultipleTasksNoContinuations
    }
}

impl TaskAdderHasTasksTrait<ContinuationAdderMultipleTasksMultipleContinuations,
                            ContinuationAdderMultipleTasksOneContinuation,
                            ContinuationAdderMultipleTasksMultipleContinuations,
                            ContinuationAdderMultipleTasksOneContinuation,
                            TaskAdderMultipleTasks> for TaskAdderMultipleTasks {
    fn add_task<TTask: 'static + Task>(self, task: TTask) -> TaskAdderMultipleTasks {
        self
    }

    fn add_task_box(self, task_box: TaskBox) -> TaskAdderMultipleTasks {
        self
    }

    fn add_task_boxes<TTaskBoxIntoIterator: 'static + TaskBoxIntoIterator>(self, task_boxes: TTaskBoxIntoIterator) -> TaskAdderMultipleTasks {
        self
    }
}

impl ContinuationAdderTrait<ContinuationAdderMultipleTasksMultipleContinuations,
                            ContinuationAdderMultipleTasksOneContinuation> for TaskAdderMultipleTasks {
    fn add_continuation<TTask: 'static + Task>(self, continuation: TTask) -> ContinuationAdderMultipleTasksOneContinuation {
        self.convert_to_continuation_adder_multiple_tasks_one_continuation()
    }

    fn add_continuation_box(self, continuation_box: TaskBox) -> ContinuationAdderMultipleTasksOneContinuation {
        self.convert_to_continuation_adder_multiple_tasks_one_continuation()
    }

    fn add_continuation_boxes<TTaskBoxIntoIterator: 'static + TaskBoxIntoIterator>(self, continuation_boxes: TTaskBoxIntoIterator) -> ContinuationAdderMultipleTasksMultipleContinuations {
        self.convert_to_continuation_adder_multiple_tasks_multiple_continuations()
            .add_continuation_boxes(continuation_boxes)
    }

    fn add_loose_continuation(self, loose_continuation: LooseContinuation) -> ContinuationAdderMultipleTasksOneContinuation {
        self.convert_to_continuation_adder_multiple_tasks_one_continuation()
    }

    fn add_loose_continuations<TLooseContinuationIntoIterator: 'static + LooseContinuationIntoIterator>(self, loose_continuations: TLooseContinuationIntoIterator) -> ContinuationAdderMultipleTasksMultipleContinuations {
        self.convert_to_continuation_adder_multiple_tasks_multiple_continuations()
            .add_loose_continuations(loose_continuations)
    }
}

impl ScheduleTrait for TaskAdderMultipleTasks {
    fn schedule(self) {
        self.convert_to_schedule_multiple_tasks_no_continuations()
            .schedule();
    }
}
