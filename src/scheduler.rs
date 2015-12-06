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
    fn convert_to_empty_task_adder<'a>(&'a self) -> EmptyTaskAdder<'a,
                                                                   Scheduler> {
        EmptyTaskAdder::new(self)
    }
}

impl <'a> BeginScheduleTrait<'a,
                             ContinuationAdderMultipleTaskBoxesMultipleContinuationBoxes<'a,
                                                                                         Scheduler>,
                             ContinuationAdderMultipleTaskBoxesOneContinuationBox<'a,
                                                                                  Scheduler>,
                             ContinuationAdderOneTaskBoxMultipleContinuationBoxes<'a,
                                                                                  Scheduler>,
                             ContinuationAdderOneTaskBoxOneContinuationBox<'a,
                                                                           Scheduler>,
                             EmptyTaskAdder<'a,
                                            Scheduler>,
                             TaskAdderMultipleTaskBoxes<'a,
                                                        Scheduler>,
                             TaskAdderOneTaskBox<'a,
                                                 Scheduler>> for Scheduler {
    fn begin_schedule(&'a self) -> EmptyTaskAdder<'a,
                                                  Scheduler> {
        self.convert_to_empty_task_adder()
    }
}

impl SchedulerTrait for Scheduler {
    type TScheduleReturn = ();

    fn schedule(&self) -> Self::TScheduleReturn {
        ()
    }
}
