use TaskBox;
use TaskBoxIntoIterator;

pub trait SchedulerTrait {
    type TTaskBoxParam;
    type TScheduleReturn;
    type TScheduleMultipleReturn;

    fn schedule(&self, task_box: TaskBox<Self::TTaskBoxParam>) -> Self::TScheduleReturn;

    fn schedule_multiple<TTaskBoxIntoIterator>(&self,
                                               task_boxes: TTaskBoxIntoIterator) -> Self::TScheduleMultipleReturn
        where TTaskBoxIntoIterator: TaskBoxIntoIterator<Self::TTaskBoxParam>;
}
