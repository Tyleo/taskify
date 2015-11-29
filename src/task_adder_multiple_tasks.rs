use ContinuationAdderMultipleTasksMultipleContinuations;
use ContinuationAdderMultipleTasksOneContinuation;
use ContinuationAdderTrait;
use LooseContinuation;
use LooseContinuationIntoIterator;
use ScheduleMultipleTasksNoContinuations;
use Scheduler;
use ScheduleTrait;
use Task;
use TaskAdderHasTasksTrait;
use TaskBox;
use TaskBoxIntoIterator;

pub struct TaskAdderMultipleTasks<'a> {
    scheduler: &'a Scheduler,
}

impl <'a> TaskAdderMultipleTasks<'a> {
    pub fn new(scheduler: &'a Scheduler) -> TaskAdderMultipleTasks<'a>  {
        TaskAdderMultipleTasks { scheduler: scheduler }
    }

    fn convert_to_continuation_adder_multiple_tasks_multiple_continuations(self) -> ContinuationAdderMultipleTasksMultipleContinuations<'a> {
        ContinuationAdderMultipleTasksMultipleContinuations::new(self.scheduler)
    }

    fn convert_to_continuation_adder_multiple_tasks_one_continuation(self) -> ContinuationAdderMultipleTasksOneContinuation<'a> {
        ContinuationAdderMultipleTasksOneContinuation::new(self.scheduler)
    }

    fn convert_to_schedule_multiple_tasks_no_continuations(self) -> ScheduleMultipleTasksNoContinuations<'a> {
        ScheduleMultipleTasksNoContinuations::new(self.scheduler)
    }
}

impl <'a> TaskAdderHasTasksTrait<ContinuationAdderMultipleTasksMultipleContinuations<'a>,
                                 ContinuationAdderMultipleTasksOneContinuation<'a>,
                                 ContinuationAdderMultipleTasksMultipleContinuations<'a>,
                                 ContinuationAdderMultipleTasksOneContinuation<'a>,
                                 TaskAdderMultipleTasks<'a>> for TaskAdderMultipleTasks<'a> {
    fn add_task<TTask: 'static + Task>(self, task: TTask) -> TaskAdderMultipleTasks<'a> {
        self
    }

    fn add_task_box(self, task_box: TaskBox) -> TaskAdderMultipleTasks<'a> {
        self
    }

    fn add_task_boxes<TTaskBoxIntoIterator: 'static + TaskBoxIntoIterator>(self, task_boxes: TTaskBoxIntoIterator) -> TaskAdderMultipleTasks<'a> {
        self
    }
}

impl <'a> ContinuationAdderTrait<ContinuationAdderMultipleTasksMultipleContinuations<'a>,
                                 ContinuationAdderMultipleTasksOneContinuation<'a>> for TaskAdderMultipleTasks<'a> {
    fn add_continuation<TTask: 'static + Task>(self, continuation: TTask) -> ContinuationAdderMultipleTasksOneContinuation<'a> {
        self.convert_to_continuation_adder_multiple_tasks_one_continuation()
    }

    fn add_continuation_box(self, continuation_box: TaskBox) -> ContinuationAdderMultipleTasksOneContinuation<'a> {
        self.convert_to_continuation_adder_multiple_tasks_one_continuation()
    }

    fn add_continuation_boxes<TTaskBoxIntoIterator: 'static + TaskBoxIntoIterator>(self, continuation_boxes: TTaskBoxIntoIterator) -> ContinuationAdderMultipleTasksMultipleContinuations<'a> {
        self.convert_to_continuation_adder_multiple_tasks_multiple_continuations()
            .add_continuation_boxes(continuation_boxes)
    }

    fn add_loose_continuation(self, loose_continuation: LooseContinuation) -> ContinuationAdderMultipleTasksOneContinuation<'a> {
        self.convert_to_continuation_adder_multiple_tasks_one_continuation()
    }

    fn add_loose_continuations<TLooseContinuationIntoIterator: 'static + LooseContinuationIntoIterator>(self, loose_continuations: TLooseContinuationIntoIterator) -> ContinuationAdderMultipleTasksMultipleContinuations<'a> {
        self.convert_to_continuation_adder_multiple_tasks_multiple_continuations()
            .add_loose_continuations(loose_continuations)
    }
}

impl <'a> ScheduleTrait for TaskAdderMultipleTasks<'a> {
    fn schedule(self) {
        self.convert_to_schedule_multiple_tasks_no_continuations()
            .schedule();
    }
}
