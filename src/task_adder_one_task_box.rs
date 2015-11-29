use ContinuationAdderMultipleTasksMultipleContinuations;
use ContinuationAdderMultipleTasksOneContinuation;
use ContinuationAdderOneTaskMultipleContinuations;
use ContinuationAdderOneTaskOneContinuation;
use ContinuationAdderTrait;
use EndScheduleOneTaskNoContinuations;
use Scheduler;
use EndScheduleTrait;
use Task;
use TaskAdderTrait;
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

    fn convert_to_task_adder_multiple_tasks(self) -> TaskAdderMultipleTasks<'a> {
        let task_boxes = vec![self.task_box];
        TaskAdderMultipleTasks::new(self.scheduler,
                                    task_boxes)
    }

    fn convert_to_continuation_adder_one_task_multiple_continuations(self) -> ContinuationAdderOneTaskMultipleContinuations<'a> {
        ContinuationAdderOneTaskMultipleContinuations::new(self.scheduler,
                                                           self.task_box,
                                                           Vec::new())
    }

    fn convert_to_continuation_adder_one_task_one_continuation(self,
                                                               continuation_box: TaskBox) -> ContinuationAdderOneTaskOneContinuation<'a> {
        ContinuationAdderOneTaskOneContinuation::new(self.scheduler,
                                                     self.task_box,
                                                     continuation_box)
    }

    fn convert_to_end_schedule_one_task_box_no_continuation_boxes(self) -> EndScheduleOneTaskNoContinuations<'a> {
        EndScheduleOneTaskNoContinuations::new(self.scheduler,
                                               self.task_box)
    }
}

impl <'a> TaskAdderTrait<ContinuationAdderMultipleTasksMultipleContinuations<'a>,
                         ContinuationAdderMultipleTasksOneContinuation<'a>,
                         ContinuationAdderOneTaskMultipleContinuations<'a>,
                         ContinuationAdderOneTaskOneContinuation<'a>,
                         TaskAdderMultipleTasks<'a>> for TaskAdderOneTask<'a> {
    fn add_task<TTask>(self,
                       task: TTask) -> TaskAdderMultipleTasks<'a>
        where TTask: 'static +
                     Task {
        let task_box = Box::new(task);
        self.add_task_box(task_box)
    }

    fn add_task_box(self,
                    task_box: TaskBox) -> TaskAdderMultipleTasks<'a> {
        self.convert_to_task_adder_multiple_tasks()
            .add_task_box(task_box)
    }

    fn add_task_boxes<TTaskBoxIntoIterator>(self,
                                            task_boxes: TTaskBoxIntoIterator) -> TaskAdderMultipleTasks<'a> 
        where TTaskBoxIntoIterator: 'static +
                                    TaskBoxIntoIterator {
        self.convert_to_task_adder_multiple_tasks()
            .add_task_boxes(task_boxes)
    }
}

impl <'a> ContinuationAdderTrait<ContinuationAdderOneTaskMultipleContinuations<'a>,
                                 ContinuationAdderOneTaskOneContinuation<'a>> for TaskAdderOneTask<'a> {
    fn add_continuation<TTask>(self,
                               continuation: TTask) -> ContinuationAdderOneTaskOneContinuation<'a>
        where TTask: 'static +
                     Task {
        let continuation_box = Box::new(continuation);
        self.add_continuation_box(continuation_box)
    }

    fn add_continuation_box(self,
                            continuation_box: TaskBox) -> ContinuationAdderOneTaskOneContinuation<'a> {
        self.convert_to_continuation_adder_one_task_one_continuation(continuation_box)
    }

    fn add_continuation_boxes<TTaskBoxIntoIterator>(self,
                                                    continuation_boxes: TTaskBoxIntoIterator) -> ContinuationAdderOneTaskMultipleContinuations<'a>
        where TTaskBoxIntoIterator: 'static +
                                    TaskBoxIntoIterator {
        self.convert_to_continuation_adder_one_task_multiple_continuations()
            .add_continuation_boxes(continuation_boxes)
    }
}

impl <'a> EndScheduleTrait for TaskAdderOneTask<'a> {
    fn end_schedule(self) {
        self.convert_to_end_schedule_one_task_box_no_continuation_boxes()
            .end_schedule();
    }
}
