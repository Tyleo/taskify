use ContinuationAdderMultipleTaskBoxesMultipleContinuationBoxes;
use ContinuationAdderMultipleTaskBoxesOneContinuationBox;
use ContinuationAdderOneTaskBoxMultipleContinuationBoxes;
use ContinuationAdderOneTaskBoxOneContinuationBox;
use ContinuationAdderTrait;
use EndScheduleOneTaskBoxNoContinuations;
use EndScheduleTrait;
use Scheduler;
use Task;
use TaskAdderTrait;
use TaskAdderMultipleTaskBoxes;
use TaskBox;
use TaskBoxIntoIterator;

pub struct TaskAdderOneTaskBox<'a> {
    scheduler: &'a Scheduler,
    task_box: TaskBox,
}

impl <'a> TaskAdderOneTaskBox<'a> {
    pub fn new(scheduler: &'a Scheduler,
               task_box: TaskBox) -> TaskAdderOneTaskBox<'a> {
        TaskAdderOneTaskBox { scheduler: scheduler,
                              task_box: task_box }
    }

    fn convert_to_task_adder_multiple_tasks(self) -> TaskAdderMultipleTaskBoxes<'a> {
        let task_boxes = vec![self.task_box];
        TaskAdderMultipleTaskBoxes::new(self.scheduler,
                                        task_boxes)
    }

    fn convert_to_continuation_adder_one_task_multiple_continuations(self) -> ContinuationAdderOneTaskBoxMultipleContinuationBoxes<'a> {
        ContinuationAdderOneTaskBoxMultipleContinuationBoxes::new(self.scheduler,
                                                                  self.task_box,
                                                           Vec::new())
    }

    fn convert_to_continuation_adder_one_task_one_continuation(self,
                                                               continuation_box: TaskBox) -> ContinuationAdderOneTaskBoxOneContinuationBox<'a> {
        ContinuationAdderOneTaskBoxOneContinuationBox::new(self.scheduler,
                                                           self.task_box,
                                                           continuation_box)
    }

    fn convert_to_end_schedule_one_task_box_no_continuation_boxes(self) -> EndScheduleOneTaskBoxNoContinuations<'a> {
        EndScheduleOneTaskBoxNoContinuations::new(self.scheduler,
                                                  self.task_box)
    }
}

impl <'a> TaskAdderTrait<ContinuationAdderMultipleTaskBoxesMultipleContinuationBoxes<'a>,
                         ContinuationAdderMultipleTaskBoxesOneContinuationBox<'a>,
                         ContinuationAdderOneTaskBoxMultipleContinuationBoxes<'a>,
                         ContinuationAdderOneTaskBoxOneContinuationBox<'a>,
                         TaskAdderMultipleTaskBoxes<'a>> for TaskAdderOneTaskBox<'a> {
    fn add_task<TTask>(self,
                       task: TTask) -> TaskAdderMultipleTaskBoxes<'a>
        where TTask: 'static +
                     Task {
        let task_box = Box::new(task);
        self.add_task_box(task_box)
    }

    fn add_task_box(self,
                    task_box: TaskBox) -> TaskAdderMultipleTaskBoxes<'a> {
        self.convert_to_task_adder_multiple_tasks()
            .add_task_box(task_box)
    }

    fn add_task_boxes<TTaskBoxIntoIterator>(self,
                                            task_boxes: TTaskBoxIntoIterator) -> TaskAdderMultipleTaskBoxes<'a> 
        where TTaskBoxIntoIterator: 'static +
                                    TaskBoxIntoIterator {
        self.convert_to_task_adder_multiple_tasks()
            .add_task_boxes(task_boxes)
    }
}

impl <'a> ContinuationAdderTrait<ContinuationAdderOneTaskBoxMultipleContinuationBoxes<'a>,
                                 ContinuationAdderOneTaskBoxOneContinuationBox<'a>> for TaskAdderOneTaskBox<'a> {
    fn add_continuation<TTask>(self,
                               continuation: TTask) -> ContinuationAdderOneTaskBoxOneContinuationBox<'a>
        where TTask: 'static +
                     Task {
        let continuation_box = Box::new(continuation);
        self.add_continuation_box(continuation_box)
    }

    fn add_continuation_box(self,
                            continuation_box: TaskBox) -> ContinuationAdderOneTaskBoxOneContinuationBox<'a> {
        self.convert_to_continuation_adder_one_task_one_continuation(continuation_box)
    }

    fn add_continuation_boxes<TTaskBoxIntoIterator>(self,
                                                    continuation_boxes: TTaskBoxIntoIterator) -> ContinuationAdderOneTaskBoxMultipleContinuationBoxes<'a>
        where TTaskBoxIntoIterator: 'static +
                                    TaskBoxIntoIterator {
        self.convert_to_continuation_adder_one_task_multiple_continuations()
            .add_continuation_boxes(continuation_boxes)
    }
}

impl <'a> EndScheduleTrait<()> for TaskAdderOneTaskBox<'a> {
    fn end_schedule(self) {
        self.convert_to_end_schedule_one_task_box_no_continuation_boxes()
            .end_schedule();
    }
}
