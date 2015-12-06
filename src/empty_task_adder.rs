use ContinuationAdderMultipleTaskBoxesMultipleContinuationBoxes;
use ContinuationAdderMultipleTaskBoxesOneContinuationBox;
use ContinuationAdderOneTaskBoxMultipleContinuationBoxes;
use ContinuationAdderOneTaskBoxOneContinuationBox;
use EmptyTaskAdderTrait;
use SchedulerTrait;
use Task;
use TaskAdderMultipleTaskBoxes;
use TaskAdderOneTaskBox;
use TaskAdderTrait;
use TaskBox;
use TaskBoxIntoIterator;

pub struct EmptyTaskAdder<'a,
                          TScheduler>
    where TScheduler: 'a +
                      SchedulerTrait {
    scheduler: &'a TScheduler,
}

impl <'a,
      TScheduler> EmptyTaskAdder<'a,
                                 TScheduler>
    where TScheduler: SchedulerTrait {
    pub fn new(scheduler: &'a TScheduler) -> EmptyTaskAdder<'a,
                                                            TScheduler> {
        EmptyTaskAdder { scheduler: scheduler }
    }

    fn convert_to_task_adder_multiple_task_boxes(self) -> TaskAdderMultipleTaskBoxes<'a,
                                                                                     TScheduler> {
        TaskAdderMultipleTaskBoxes::new(self.scheduler,
                                        Vec::new())
    }

    fn convert_to_task_adder_one_task_box(self,
                                          task_box: TaskBox) -> TaskAdderOneTaskBox<'a,
                                                                                    TScheduler> {
        TaskAdderOneTaskBox::new(self.scheduler,
                                 task_box)
    }
}

impl <'a,
      TScheduler> EmptyTaskAdderTrait<ContinuationAdderMultipleTaskBoxesMultipleContinuationBoxes<'a,
                                                                                                  TScheduler>,
                                      ContinuationAdderMultipleTaskBoxesOneContinuationBox<'a,
                                                                                           TScheduler>,
                                      ContinuationAdderOneTaskBoxMultipleContinuationBoxes<'a,
                                                                                           TScheduler>,
                                      ContinuationAdderOneTaskBoxOneContinuationBox<'a,
                                                                                    TScheduler>,
                                      TaskAdderMultipleTaskBoxes<'a,
                                                                 TScheduler>,
                                      TaskAdderOneTaskBox<'a,
                                                          TScheduler>> for EmptyTaskAdder<'a,
                                                                                          TScheduler>
    where TScheduler: SchedulerTrait {
    fn add_task<TTask>(self,
                       task: TTask) -> TaskAdderOneTaskBox<'a,
                                                           TScheduler>
        where TTask: 'static +
                     Task {
        let task_box = Box::new(task);
        self.add_task_box(task_box)
    }

    fn add_task_box(self,
                    task_box: TaskBox) -> TaskAdderOneTaskBox<'a,
                                                              TScheduler> {
        self.convert_to_task_adder_one_task_box(task_box)
    }

    fn add_task_boxes<TTaskBoxIntoIterator>(self,
                                            task_boxes: TTaskBoxIntoIterator) -> TaskAdderMultipleTaskBoxes<'a,
                                                                                                            TScheduler>
        where TTaskBoxIntoIterator: 'static +
                                    TaskBoxIntoIterator {
        self.convert_to_task_adder_multiple_task_boxes()
            .add_task_boxes(task_boxes)
    }
}
