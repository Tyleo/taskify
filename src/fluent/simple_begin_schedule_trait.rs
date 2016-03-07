use fluent::BeginScheduleTrait;
use fluent::ContinuationAdderMultipleTaskBoxesMultipleContinuationBoxes;
use fluent::ContinuationAdderMultipleTaskBoxesOneContinuationBox;
use fluent::ContinuationAdderOneTaskBoxMultipleContinuationBoxes;
use fluent::ContinuationAdderOneTaskBoxOneContinuationBox;
use fluent::EmptyTaskAdder;
use SchedulerTrait;
use fluent::TaskAdderMultipleTaskBoxes;
use fluent::TaskAdderOneTaskBox;

pub trait SimpleBeginScheduleTrait<'a,
                                   TScheduler>
    where TScheduler: 'a +
                      SchedulerTrait { }

impl <'a,
      TScheduler> BeginScheduleTrait<'a,
                                     TScheduler,
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
