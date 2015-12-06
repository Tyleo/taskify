use ContinuationAdderMultipleTaskBoxesMultipleContinuationBoxes;
use ContinuationAdderMultipleTaskBoxesOneContinuationBox;
use ContinuationAdderOneTaskBoxMultipleContinuationBoxes;
use ContinuationAdderOneTaskBoxOneContinuationBox;
use EmptyTaskAdderTrait;
use Scheduler;
use Task;
use TaskAdderMultipleTaskBoxes;
use TaskAdderOneTaskBox;
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

    fn convert_to_task_adder_multiple_task_boxes(self) -> TaskAdderMultipleTaskBoxes<'a> {
        TaskAdderMultipleTaskBoxes::new(self.scheduler,
                                        Vec::new())
    }

    fn convert_to_task_adder_one_task_box(self,
                                          task_box: TaskBox) -> TaskAdderOneTaskBox<'a> {
        TaskAdderOneTaskBox::new(self.scheduler,
                                 task_box)
    }
}

impl <'a> EmptyTaskAdderTrait<ContinuationAdderMultipleTaskBoxesMultipleContinuationBoxes<'a>,
                              ContinuationAdderMultipleTaskBoxesOneContinuationBox<'a>,
                              ContinuationAdderOneTaskBoxMultipleContinuationBoxes<'a>,
                              ContinuationAdderOneTaskBoxOneContinuationBox<'a>,
                              TaskAdderMultipleTaskBoxes<'a>,
                              TaskAdderOneTaskBox<'a>> for EmptyTaskAdder<'a> {
    fn add_task<TTask>(self,
                       task: TTask) -> TaskAdderOneTaskBox<'a>
        where TTask: 'static +
                     Task {
        let task_box = Box::new(task);
        self.add_task_box(task_box)
    }

    fn add_task_box(self,
                    task_box: TaskBox) -> TaskAdderOneTaskBox<'a> {
        self.convert_to_task_adder_one_task_box(task_box)
    }

    fn add_task_boxes<TTaskBoxIntoIterator>(self,
                                            task_boxes: TTaskBoxIntoIterator) -> TaskAdderMultipleTaskBoxes<'a>
        where TTaskBoxIntoIterator: 'static +
                                    TaskBoxIntoIterator {
        self.convert_to_task_adder_multiple_task_boxes()
            .add_task_boxes(task_boxes)
    }
}
