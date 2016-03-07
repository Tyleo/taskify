use fluent::ContinuationAdderMultipleTaskBoxesMultipleContinuationBoxes;
use fluent::ContinuationAdderMultipleTaskBoxesOneContinuationBox;
use fluent::ContinuationAdderTrait;
use fluent::EndScheduleMultipleTaskBoxesNoContinuations;
use fluent::EndScheduleTrait;
use SchedulerTrait;
use Task;
use fluent::TaskAdderTrait;
use TaskBox;
use TaskBoxIntoIterator;

pub struct TaskAdderMultipleTaskBoxes<'a,
                                      TScheduler>
    where TScheduler: 'a +
                      SchedulerTrait {
    scheduler: &'a TScheduler,
    task_boxes: Vec<TaskBox<TScheduler::TTaskBoxParam>>,
}

impl <'a,
      TScheduler> TaskAdderMultipleTaskBoxes<'a,
                                             TScheduler>
    where TScheduler: SchedulerTrait {
    pub fn new(scheduler: &'a TScheduler,
               task_boxes: Vec<TaskBox<TScheduler::TTaskBoxParam>>) -> TaskAdderMultipleTaskBoxes<'a,
                                                                                                  TScheduler> {
        TaskAdderMultipleTaskBoxes { scheduler: scheduler,
                                     task_boxes: task_boxes }
    }

    fn convert_to_continuation_adder_multiple_tasks_multiple_continuations(self) -> ContinuationAdderMultipleTaskBoxesMultipleContinuationBoxes<'a,
                                                                                                                                                TScheduler> {
        ContinuationAdderMultipleTaskBoxesMultipleContinuationBoxes::new(self.scheduler,
                                                                         self.task_boxes,
                                                                         Vec::new())
    }

    fn convert_to_continuation_adder_multiple_tasks_one_continuation(self,
                                                                     continuation_box: TaskBox<TScheduler::TTaskBoxParam>) -> ContinuationAdderMultipleTaskBoxesOneContinuationBox<'a,
                                                                                                                                                                                   TScheduler> {
        ContinuationAdderMultipleTaskBoxesOneContinuationBox::new(self.scheduler,
                                                                  self.task_boxes,
                                                                  continuation_box)
    }

    fn convert_to_end_schedule_multiple_task_boxes_no_continuation_boxes(self) -> EndScheduleMultipleTaskBoxesNoContinuations<'a,
                                                                                                                              TScheduler> {
        EndScheduleMultipleTaskBoxesNoContinuations::new(self.scheduler,
                                                         self.task_boxes)
    }
}

impl <'a,
      TScheduler> TaskAdderTrait<TScheduler,
                                 ContinuationAdderMultipleTaskBoxesMultipleContinuationBoxes<'a,
                                                                                             TScheduler>,
                                 ContinuationAdderMultipleTaskBoxesOneContinuationBox<'a,
                                                                                      TScheduler>,
                                 ContinuationAdderMultipleTaskBoxesMultipleContinuationBoxes<'a,
                                                                                             TScheduler>,
                                 ContinuationAdderMultipleTaskBoxesOneContinuationBox<'a,
                                                                                      TScheduler>,
                                 TaskAdderMultipleTaskBoxes<'a,
                                                            TScheduler>> for TaskAdderMultipleTaskBoxes<'a,
                                                                                                        TScheduler>
    where TScheduler: SchedulerTrait {
    fn add_task<TTask>(self,
                       task: TTask) -> TaskAdderMultipleTaskBoxes<'a,
                                                                  TScheduler>
        where TTask: 'static +
                     Task<TScheduler::TTaskBoxParam> {
        let task_box = Box::new(task);
        self.add_task_box(task_box)
    }

    fn add_task_box(self,
                    task_box: TaskBox<TScheduler::TTaskBoxParam>) -> TaskAdderMultipleTaskBoxes<'a,
                                                                                                TScheduler> {
        let mut mut_self = self;
        mut_self.task_boxes.push(task_box);
        mut_self
    }

    fn add_task_boxes<TTaskBoxIntoIterator>(self,
                                            task_boxes: TTaskBoxIntoIterator) -> TaskAdderMultipleTaskBoxes<'a,
                                                                                                            TScheduler>
        where TTaskBoxIntoIterator: TaskBoxIntoIterator<TScheduler::TTaskBoxParam> {
        let mut mut_self = self;
        for task_box in task_boxes {
            mut_self.task_boxes.push(task_box);
        }
        mut_self
    }
}

impl <'a,
      TScheduler> ContinuationAdderTrait<TScheduler,
                                         ContinuationAdderMultipleTaskBoxesMultipleContinuationBoxes<'a,
                                                                                                     TScheduler>,
                                         ContinuationAdderMultipleTaskBoxesOneContinuationBox<'a,
                                                                                              TScheduler>> for TaskAdderMultipleTaskBoxes<'a,
                                                                                                                                          TScheduler>
    where TScheduler: SchedulerTrait {
    fn add_continuation<TTask>(self,
                               continuation: TTask) -> ContinuationAdderMultipleTaskBoxesOneContinuationBox<'a,
                                                                                                            TScheduler>
        where TTask: 'static +
                     Task<TScheduler::TTaskBoxParam> {
        let continuation_box = Box::new(continuation);
        self.add_continuation_box(continuation_box)
    }

    fn add_continuation_box(self,
                            continuation_box: TaskBox<TScheduler::TTaskBoxParam>) -> ContinuationAdderMultipleTaskBoxesOneContinuationBox<'a,
                                                                                                                                          TScheduler> {
        self.convert_to_continuation_adder_multiple_tasks_one_continuation(continuation_box)
    }

    fn add_continuation_boxes<TTaskBoxIntoIterator>(self,
                                                    continuation_boxes: TTaskBoxIntoIterator) -> ContinuationAdderMultipleTaskBoxesMultipleContinuationBoxes<'a,
                                                                                                                                                             TScheduler>
        where TTaskBoxIntoIterator: 'static +
                                    TaskBoxIntoIterator<TScheduler::TTaskBoxParam> {
        self.convert_to_continuation_adder_multiple_tasks_multiple_continuations()
            .add_continuation_boxes(continuation_boxes)
    }
}

impl <'a,
      TScheduler> EndScheduleTrait for TaskAdderMultipleTaskBoxes<'a,
                                                                  TScheduler>
    where TScheduler: SchedulerTrait {
    type TEndScheduleReturn = TScheduler::TScheduleMultipleReturn;

    fn end_schedule(self) -> Self::TEndScheduleReturn {
        self.convert_to_end_schedule_multiple_task_boxes_no_continuation_boxes()
            .end_schedule()
    }
}
