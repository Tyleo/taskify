use fluent::BeginScheduleTrait;
use fluent::EmptyTaskAdder;
use SchedulerTrait;
use fluent::SimpleBeginScheduleTrait;
use std::marker::PhantomData;
use TaskBox;
use TaskBoxIntoIterator;

pub struct LooseContinuation<TScheduler>
    where TScheduler: SchedulerTrait {
    _scheduler: PhantomData<TScheduler>,
}

impl <TScheduler> LooseContinuation<TScheduler>
    where TScheduler: SchedulerTrait {
    pub fn new() -> LooseContinuation<TScheduler> {
        LooseContinuation::<TScheduler>{ _scheduler: PhantomData, }
    }
}

impl <TScheduler> SchedulerTrait for LooseContinuation<TScheduler>
    where TScheduler: SchedulerTrait {
    type TTaskBoxParam = TScheduler::TTaskBoxParam;
    type TScheduleReturn = TaskBox<Self::TTaskBoxParam>;
    type TScheduleMultipleReturn = TaskBox<Self::TTaskBoxParam>;

    fn schedule(&self,
                task_box: TaskBox<Self::TTaskBoxParam>) -> Self::TScheduleReturn {
        task_box
    }

    fn schedule_multiple<TTaskBoxIntoIterator>(&self,
                                               task_boxes: TTaskBoxIntoIterator) -> Self::TScheduleMultipleReturn
        where TTaskBoxIntoIterator: 'static +
                                    TaskBoxIntoIterator<Self::TTaskBoxParam> {
        Box::new(
            move |scheduler: &Self::TTaskBoxParam| {
                scheduler.schedule_multiple(task_boxes);
            }
        )
    }
}
