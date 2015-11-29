use ContinuationAdderMultipleTasksMultipleContinuations;
use ContinuationAdderMultipleTasksOneContinuation;
use ContinuationAdderOneTaskMultipleContinuations;
use ContinuationAdderOneTaskOneContinuation;
use ContinuationAdderTrait;
use LooseContinuation;
use LooseContinuationIntoIterator;
use ScheduleOneTaskNoContinuations;
use Scheduler;
use ScheduleTrait;
use std::iter::FromIterator;
use Task;
use TaskAdderHasTasksTrait;
use TaskAdderMultipleTasks;
use TaskBox;
use TaskBoxIntoIterator;

pub struct TaskAdderOneTask<'a> {
    scheduler: &'a Scheduler,
    task_box: TaskBox,
}

impl <'a> TaskAdderOneTask<'a> {
    pub fn new(scheduler: &'a Scheduler,
               task_box: TaskBox) -> TaskAdderOneTask<'a> {
        TaskAdderOneTask { scheduler: scheduler,
                           task_box: task_box }
    }

    fn convert_to_task_adder_multiple_tasks(self,
                                            task_boxes: Vec<TaskBox>) -> TaskAdderMultipleTasks<'a> {
        TaskAdderMultipleTasks::new(self.scheduler,
                                    task_boxes)
    }

    fn convert_to_continuation_adder_one_task_multiple_continuations(self) -> ContinuationAdderOneTaskMultipleContinuations<'a> {
        ContinuationAdderOneTaskMultipleContinuations::new(self.scheduler)
    }

    fn convert_to_continuation_adder_one_task_one_continuation(self) -> ContinuationAdderOneTaskOneContinuation<'a> {
        ContinuationAdderOneTaskOneContinuation::new(self.scheduler)
    }

    fn convert_to_schedule_one_task_no_continuations(self) -> ScheduleOneTaskNoContinuations<'a> {
        ScheduleOneTaskNoContinuations::new(self.scheduler)
    }
}

impl <'a> TaskAdderHasTasksTrait<ContinuationAdderMultipleTasksMultipleContinuations<'a>,
                                 ContinuationAdderMultipleTasksOneContinuation<'a>,
                                 ContinuationAdderOneTaskMultipleContinuations<'a>,
                                 ContinuationAdderOneTaskOneContinuation<'a>,
                                 TaskAdderMultipleTasks<'a>> for TaskAdderOneTask<'a> {
    fn add_task<TTask: 'static + Task>(self, task: TTask) -> TaskAdderMultipleTasks<'a> {
        let task_box = Box::new(task);
        self.add_task_box(task_box)
    }

    fn add_task_box(self, task_box: TaskBox) -> TaskAdderMultipleTasks<'a> {
        let task_boxes = vec![task_box];
        self.convert_to_task_adder_multiple_tasks(task_boxes)
    }

    fn add_task_boxes<TTaskBoxIntoIterator: 'static + TaskBoxIntoIterator>(self, task_boxes: TTaskBoxIntoIterator) -> TaskAdderMultipleTasks<'a> {
        let task_boxes_vec = Vec::from_iter(task_boxes);
        self.convert_to_task_adder_multiple_tasks(task_boxes_vec)
    }
}

impl <'a> ContinuationAdderTrait<ContinuationAdderOneTaskMultipleContinuations<'a>,
                                 ContinuationAdderOneTaskOneContinuation<'a>> for TaskAdderOneTask<'a> {
    fn add_continuation<TTask: 'static + Task>(self, continuation: TTask) -> ContinuationAdderOneTaskOneContinuation<'a> {
        self.convert_to_continuation_adder_one_task_one_continuation()
    }

    fn add_continuation_box(self, continuation_box: TaskBox) -> ContinuationAdderOneTaskOneContinuation<'a> {
        self.convert_to_continuation_adder_one_task_one_continuation()
    }

    fn add_continuation_boxes<TTaskBoxIntoIterator: 'static + TaskBoxIntoIterator>(self, continuation_boxes: TTaskBoxIntoIterator) -> ContinuationAdderOneTaskMultipleContinuations<'a> {
        self.convert_to_continuation_adder_one_task_multiple_continuations()
            .add_continuation_boxes(continuation_boxes)
    }

    fn add_loose_continuation(self, loose_continuation: LooseContinuation) -> ContinuationAdderOneTaskOneContinuation<'a> {
        self.convert_to_continuation_adder_one_task_one_continuation()
    }

    fn add_loose_continuations<TLooseContinuationIntoIterator: 'static + LooseContinuationIntoIterator>(self, loose_continuations: TLooseContinuationIntoIterator) -> ContinuationAdderOneTaskMultipleContinuations<'a> {
        self.convert_to_continuation_adder_one_task_multiple_continuations()
            .add_loose_continuations(loose_continuations)
    }
}

impl <'a> ScheduleTrait for TaskAdderOneTask<'a> {
    fn schedule(self) {
        self.convert_to_schedule_one_task_no_continuations()
            .schedule();
    }
}
