use ContinuationAdderMultipleTaskBoxesMultipleContinuationBoxes;
use ContinuationAdderMultipleTaskBoxesOneContinuationBox;
use ContinuationAdderTrait;
use EndScheduleMultipleTaskBoxesNoContinuations;
use Scheduler;
use EndScheduleTrait;
use Task;
use TaskAdderTrait;
use TaskBox;
use TaskBoxIntoIterator;

pub struct TaskAdderMultipleTaskBoxes<'a> {
    scheduler: &'a Scheduler,
    task_boxes: Vec<TaskBox>,
}

impl <'a> TaskAdderMultipleTaskBoxes<'a> {
    pub fn new(scheduler: &'a Scheduler,
               task_boxes: Vec<TaskBox>) -> TaskAdderMultipleTaskBoxes<'a> {
        TaskAdderMultipleTaskBoxes { scheduler: scheduler,
                                     task_boxes: task_boxes }
    }

    fn convert_to_continuation_adder_multiple_tasks_multiple_continuations(self) -> ContinuationAdderMultipleTaskBoxesMultipleContinuationBoxes<'a> {
        ContinuationAdderMultipleTaskBoxesMultipleContinuationBoxes::new(self.scheduler,
                                                                         self.task_boxes,
                                                                         Vec::new())
    }

    fn convert_to_continuation_adder_multiple_tasks_one_continuation(self,
                                                                     continuation_box: TaskBox) -> ContinuationAdderMultipleTaskBoxesOneContinuationBox<'a> {
        ContinuationAdderMultipleTaskBoxesOneContinuationBox::new(self.scheduler,
                                                                  self.task_boxes,
                                                                  continuation_box)
    }

    fn convert_to_end_schedule_multiple_task_boxes_no_continuation_boxes(self) -> EndScheduleMultipleTaskBoxesNoContinuations<'a> {
        EndScheduleMultipleTaskBoxesNoContinuations::new(self.scheduler,
                                                             self.task_boxes)
    }
}

impl <'a> TaskAdderTrait<ContinuationAdderMultipleTaskBoxesMultipleContinuationBoxes<'a>,
                         ContinuationAdderMultipleTaskBoxesOneContinuationBox<'a>,
                         ContinuationAdderMultipleTaskBoxesMultipleContinuationBoxes<'a>,
                         ContinuationAdderMultipleTaskBoxesOneContinuationBox<'a>,
                         TaskAdderMultipleTaskBoxes<'a>> for TaskAdderMultipleTaskBoxes<'a> {
    fn add_task<TTask>(self,
                       task: TTask) -> TaskAdderMultipleTaskBoxes<'a>
        where TTask: 'static +
                     Task {
        let task_box = Box::new(task);
        self.add_task_box(task_box)
    }

    fn add_task_box(self,
                    task_box: TaskBox) -> TaskAdderMultipleTaskBoxes<'a> {
        let mut mut_self = self;
        mut_self.task_boxes.push(task_box);
        mut_self
    }

    fn add_task_boxes<TTaskBoxIntoIterator>(self,
                                            task_boxes: TTaskBoxIntoIterator) -> TaskAdderMultipleTaskBoxes<'a>
        where TTaskBoxIntoIterator: 'static +
                                    TaskBoxIntoIterator {
        let mut mut_self = self;
        for task_box in task_boxes {
            mut_self.task_boxes.push(task_box);
        }
        mut_self
    }
}

impl <'a> ContinuationAdderTrait<ContinuationAdderMultipleTaskBoxesMultipleContinuationBoxes<'a>,
                                 ContinuationAdderMultipleTaskBoxesOneContinuationBox<'a>> for TaskAdderMultipleTaskBoxes<'a> {
    fn add_continuation<TTask>(self,
                               continuation: TTask) -> ContinuationAdderMultipleTaskBoxesOneContinuationBox<'a>
        where TTask: 'static +
                     Task {
        let continuation_box = Box::new(continuation);
        self.add_continuation_box(continuation_box)
    }

    fn add_continuation_box(self,
                            continuation_box: TaskBox) -> ContinuationAdderMultipleTaskBoxesOneContinuationBox<'a> {
        self.convert_to_continuation_adder_multiple_tasks_one_continuation(continuation_box)
    }

    fn add_continuation_boxes<TTaskBoxIntoIterator>(self,
                                                    continuation_boxes: TTaskBoxIntoIterator) -> ContinuationAdderMultipleTaskBoxesMultipleContinuationBoxes<'a>
        where TTaskBoxIntoIterator: 'static +
                                    TaskBoxIntoIterator {
        self.convert_to_continuation_adder_multiple_tasks_multiple_continuations()
            .add_continuation_boxes(continuation_boxes)
    }
}

impl <'a> EndScheduleTrait<()> for TaskAdderMultipleTaskBoxes<'a> {
    fn end_schedule(self) {
        self.convert_to_end_schedule_multiple_task_boxes_no_continuation_boxes()
            .end_schedule();
    }
}
