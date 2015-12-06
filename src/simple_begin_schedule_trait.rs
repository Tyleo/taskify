use BeginScheduleTrait;
use ContinuationAdderMultipleTaskBoxesMultipleContinuationBoxes;
use ContinuationAdderMultipleTaskBoxesOneContinuationBox;
use ContinuationAdderOneTaskBoxMultipleContinuationBoxes;
use ContinuationAdderOneTaskBoxOneContinuationBox;
use EmptyTaskAdder;
use SchedulerTrait;
use TaskAdderMultipleTaskBoxes;
use TaskAdderOneTaskBox;

pub trait SimpleBeginScheduleTrait<'a,
                                   TScheduler>
    where TScheduler: 'a +
                      SchedulerTrait { }

impl <'a,
      TScheduler> BeginScheduleTrait<'a,
                                     ContinuationAdderMultipleTaskBoxesMultipleContinuationBoxes<'a,
                                                                                                 TScheduler>,
                                     ContinuationAdderMultipleTaskBoxesOneContinuationBox<'a,
                                                                                          TScheduler>,
                                     ContinuationAdderOneTaskBoxMultipleContinuationBoxes<'a,
                                                                                          TScheduler>,
                                     ContinuationAdderOneTaskBoxOneContinuationBox<'a,
                                                                                   TScheduler>,
                                     EmptyTaskAdder<'a,
                                                    TScheduler>,
                                     TaskAdderMultipleTaskBoxes<'a,
                                                                TScheduler>,
                                     TaskAdderOneTaskBox<'a,
                                                         TScheduler>> for TScheduler
    where TScheduler: SimpleBeginScheduleTrait<'a,
                                               TScheduler> +
                      SchedulerTrait {
    fn begin_schedule(&'a self) -> EmptyTaskAdder<'a,
                                                  TScheduler> {
        EmptyTaskAdder::new(self)
    }
}

