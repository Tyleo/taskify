use fluent::ContinuationAdderMultipleTaskBoxesMultipleContinuationBoxes;
use fluent::ContinuationAdderMultipleTaskBoxesOneContinuationBox;
use fluent::ContinuationAdderOneTaskBoxMultipleContinuationBoxes;
use fluent::ContinuationAdderOneTaskBoxOneContinuationBox;
use fluent::ContinuationAdderTrait;
use fluent::EndScheduleOneTaskBoxNoContinuations;
use fluent::EndScheduleTrait;
use SchedulerTrait;
use Task;
use fluent::TaskAdderTrait;
use fluent::TaskAdderMultipleTaskBoxes;
use TaskBox;
use TaskBoxIntoIterator;

pub struct TaskAdderOneTaskBox<'a,
                               TScheduler>
    where TScheduler: 'a +
                      SchedulerTrait {
    scheduler: &'a TScheduler,
    task_box: TaskBox<TScheduler::TTaskBoxParam>,
}

impl <'a,
      TScheduler> TaskAdderOneTaskBox<'a,
                                      TScheduler>
    where TScheduler: 'a +
                      SchedulerTrait {
    pub fn new(scheduler: &'a TScheduler,
               task_box: TaskBox<TScheduler::TTaskBoxParam>) -> TaskAdderOneTaskBox<'a,
                                                                TScheduler> {
        TaskAdderOneTaskBox { scheduler: scheduler,
                              task_box: task_box }
    }

    fn convert_to_task_adder_multiple_tasks(self) -> TaskAdderMultipleTaskBoxes<'a,
                                                                                TScheduler> {
        let task_boxes = vec![self.task_box];
        TaskAdderMultipleTaskBoxes::new(self.scheduler,
                                        task_boxes)
    }

    fn convert_to_continuation_adder_one_task_multiple_continuations(self) -> ContinuationAdderOneTaskBoxMultipleContinuationBoxes<'a,
                                                                                                                                   TScheduler> {
        ContinuationAdderOneTaskBoxMultipleContinuationBoxes::new(self.scheduler,
                                                                  self.task_box,
                                                           Vec::new())
    }

    fn convert_to_continuation_adder_one_task_one_continuation(self,
                                                               continuation_box: TaskBox<TScheduler::TTaskBoxParam>) -> ContinuationAdderOneTaskBoxOneContinuationBox<'a,
                                                                                                                                                                      TScheduler> {
        ContinuationAdderOneTaskBoxOneContinuationBox::new(self.scheduler,
                                                           self.task_box,
                                                           continuation_box)
    }

    fn convert_to_end_schedule_one_task_box_no_continuation_boxes(self) -> EndScheduleOneTaskBoxNoContinuations<'a,
                                                                                                                TScheduler> {
        EndScheduleOneTaskBoxNoContinuations::new(self.scheduler,
                                                  self.task_box)
    }
}

impl <'a,
      TScheduler> TaskAdderTrait<TScheduler,
                                 ContinuationAdderMultipleTaskBoxesMultipleContinuationBoxes<'a,
                                                                                             TScheduler>,
                                 ContinuationAdderMultipleTaskBoxesOneContinuationBox<'a,
                                                                                      TScheduler>,
                                 ContinuationAdderOneTaskBoxMultipleContinuationBoxes<'a,
                                                                                      TScheduler>,
                                 ContinuationAdderOneTaskBoxOneContinuationBox<'a,
                                                                               TScheduler>,
                                 TaskAdderMultipleTaskBoxes<'a,
                                                            TScheduler>> for TaskAdderOneTaskBox<'a,
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
        self.convert_to_task_adder_multiple_tasks()
            .add_task_box(task_box)
    }

    fn add_task_boxes<TTaskBoxIntoIterator>(self,
                                            task_boxes: TTaskBoxIntoIterator) -> TaskAdderMultipleTaskBoxes<'a,
                                                                                                            TScheduler>
        where TTaskBoxIntoIterator: TaskBoxIntoIterator<TScheduler::TTaskBoxParam> {
        self.convert_to_task_adder_multiple_tasks()
            .add_task_boxes(task_boxes)
    }
}

impl <'a,
      TScheduler> ContinuationAdderTrait<TScheduler,
                                         ContinuationAdderOneTaskBoxMultipleContinuationBoxes<'a,
                                                                                              TScheduler>,
                                         ContinuationAdderOneTaskBoxOneContinuationBox<'a,
                                                                                       TScheduler>> for TaskAdderOneTaskBox<'a,
                                                                                                                            TScheduler>
    where TScheduler: SchedulerTrait {
    fn add_continuation<TTask>(self,
                               continuation: TTask) -> ContinuationAdderOneTaskBoxOneContinuationBox<'a,
                                                                                                     TScheduler>
        where TTask: 'static +
                     Task<TScheduler::TTaskBoxParam> {
        let continuation_box = Box::new(continuation);
        self.add_continuation_box(continuation_box)
    }

    fn add_continuation_box(self,
                            continuation_box: TaskBox<TScheduler::TTaskBoxParam>) -> ContinuationAdderOneTaskBoxOneContinuationBox<'a,
                                                                                                                                   TScheduler> {
        self.convert_to_continuation_adder_one_task_one_continuation(continuation_box)
    }

    fn add_continuation_boxes<TTaskBoxIntoIterator>(self,
                                                    continuation_boxes: TTaskBoxIntoIterator) -> ContinuationAdderOneTaskBoxMultipleContinuationBoxes<'a,
                                                                                                                                                      TScheduler>
        where TTaskBoxIntoIterator: TaskBoxIntoIterator<TScheduler::TTaskBoxParam> {
        self.convert_to_continuation_adder_one_task_multiple_continuations()
            .add_continuation_boxes(continuation_boxes)
    }
}

impl <'a,
      TScheduler> EndScheduleTrait for TaskAdderOneTaskBox<'a,
                                                           TScheduler>
    where TScheduler: SchedulerTrait {
    type TEndScheduleReturn = TScheduler::TScheduleReturn;

    fn end_schedule(self) -> Self::TEndScheduleReturn {
        self.convert_to_end_schedule_one_task_box_no_continuation_boxes()
            .end_schedule()
    }
}
