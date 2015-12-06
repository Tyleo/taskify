use BeginScheduleTrait;
use ContinuationAdderMultipleTaskBoxesMultipleContinuationBoxes;
use ContinuationAdderMultipleTaskBoxesOneContinuationBox;
use ContinuationAdderOneTaskBoxMultipleContinuationBoxes;
use ContinuationAdderOneTaskBoxOneContinuationBox;
use EmptyTaskAdder;
use SchedulerTrait;
use TaskAdderMultipleTaskBoxes;
use TaskAdderOneTaskBox;

pub struct Scheduler;

impl Scheduler {
    fn convert_to_empty_task_adder<'a>(&'a self) -> EmptyTaskAdder<'a> {
        EmptyTaskAdder::new(self)
    }
}

impl <'a> BeginScheduleTrait<'a,
                             ContinuationAdderMultipleTaskBoxesMultipleContinuationBoxes<'a>,
                             ContinuationAdderMultipleTaskBoxesOneContinuationBox<'a>,
                             ContinuationAdderOneTaskBoxMultipleContinuationBoxes<'a>,
                             ContinuationAdderOneTaskBoxOneContinuationBox<'a>,
                             EmptyTaskAdder<'a>,
                             TaskAdderMultipleTaskBoxes<'a>,
                             TaskAdderOneTaskBox<'a>> for Scheduler {
    fn begin_schedule(&'a self) -> EmptyTaskAdder<'a> {
        self.convert_to_empty_task_adder()
    }
}

impl SchedulerTrait for Scheduler {
    type TScheduleReturn = ();

    fn schedule(&self) -> Self::TScheduleReturn {
        ()
    }
}
