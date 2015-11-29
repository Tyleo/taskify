use ContinuationAdderMultipleTasksMultipleContinuations;
use ContinuationAdderMultipleTasksOneContinuation;
use ContinuationAdderTrait;
use EndScheduleMultipleTasksNoContinuations;
use Scheduler;
use EndScheduleTrait;
use Task;
use TaskAdderTrait;
use TaskBox;
use TaskBoxIntoIterator;

pub struct TaskAdderMultipleTasks<'a> {
    scheduler: &'a Scheduler,
    task_boxes: Vec<TaskBox>,
}

impl <'a> TaskAdderMultipleTasks<'a> {
    pub fn new(scheduler: &'a Scheduler,
               task_boxes: Vec<TaskBox>) -> TaskAdderMultipleTasks<'a> {
        TaskAdderMultipleTasks { scheduler: scheduler,
                                 task_boxes: task_boxes }
    }

    fn convert_to_continuation_adder_multiple_tasks_multiple_continuations(self) -> ContinuationAdderMultipleTasksMultipleContinuations<'a> {
        ContinuationAdderMultipleTasksMultipleContinuations::new(self.scheduler,
                                                                 self.task_boxes,
                                                                 Vec::new())
    }

    fn convert_to_continuation_adder_multiple_tasks_one_continuation(self,
                                                                     continuation_box: TaskBox) -> ContinuationAdderMultipleTasksOneContinuation<'a> {
        ContinuationAdderMultipleTasksOneContinuation::new(self.scheduler,
                                                           self.task_boxes,
                                                           continuation_box)
    }

    fn convert_to_end_schedule_multiple_task_boxes_no_continuation_boxes(self) -> EndScheduleMultipleTasksNoContinuations<'a> {
        EndScheduleMultipleTasksNoContinuations::new(self.scheduler,
                                                     self.task_boxes)
    }
}

impl <'a> TaskAdderTrait<ContinuationAdderMultipleTasksMultipleContinuations<'a>,
                         ContinuationAdderMultipleTasksOneContinuation<'a>,
                         ContinuationAdderMultipleTasksMultipleContinuations<'a>,
                         ContinuationAdderMultipleTasksOneContinuation<'a>,
                         TaskAdderMultipleTasks<'a>> for TaskAdderMultipleTasks<'a> {
    fn add_task<TTask>(self,
                       task: TTask) -> TaskAdderMultipleTasks<'a>
        where TTask: 'static +
                     Task {
        let task_box = Box::new(task);
        self.add_task_box(task_box)
    }

    fn add_task_box(self,
                    task_box: TaskBox) -> TaskAdderMultipleTasks<'a> {
        let mut mut_self = self;
        mut_self.task_boxes.push(task_box);
        mut_self
    }

    fn add_task_boxes<TTaskBoxIntoIterator>(self,
                                            task_boxes: TTaskBoxIntoIterator) -> TaskAdderMultipleTasks<'a>
        where TTaskBoxIntoIterator: 'static +
                                    TaskBoxIntoIterator {
        let mut mut_self = self;
        for task_box in task_boxes {
            mut_self.task_boxes.push(task_box);
        }
        mut_self
    }
}

impl <'a> ContinuationAdderTrait<ContinuationAdderMultipleTasksMultipleContinuations<'a>,
                                 ContinuationAdderMultipleTasksOneContinuation<'a>> for TaskAdderMultipleTasks<'a> {
    fn add_continuation<TTask>(self,
                               continuation: TTask) -> ContinuationAdderMultipleTasksOneContinuation<'a>
        where TTask: 'static +
                     Task {
        let continuation_box = Box::new(continuation);
        self.add_continuation_box(continuation_box)
    }

    fn add_continuation_box(self,
                            continuation_box: TaskBox) -> ContinuationAdderMultipleTasksOneContinuation<'a> {
        self.convert_to_continuation_adder_multiple_tasks_one_continuation(continuation_box)
    }

    fn add_continuation_boxes<TTaskBoxIntoIterator>(self,
                                                    continuation_boxes: TTaskBoxIntoIterator) -> ContinuationAdderMultipleTasksMultipleContinuations<'a>
        where TTaskBoxIntoIterator: 'static +
                                    TaskBoxIntoIterator {
        self.convert_to_continuation_adder_multiple_tasks_multiple_continuations()
            .add_continuation_boxes(continuation_boxes)
    }
}

impl <'a> EndScheduleTrait for TaskAdderMultipleTasks<'a> {
    fn end_schedule(self) {
        self.convert_to_end_schedule_multiple_task_boxes_no_continuation_boxes()
            .end_schedule();
    }
}
