use ContinuationAdderTrait;
use EndScheduleTrait;
use Task;
use TaskBox;
use TaskBoxIntoIterator;

pub trait TaskAdderTrait<TContinuationAdder: ContinuationAdderTrait<TContinuationAdder>,
                         TTaskAdder: TaskAdderTrait<TContinuationAdder, TTaskAdder>>: ContinuationAdderTrait<TContinuationAdder> + EndScheduleTrait {
    fn add_task<TTask: 'static + Task>(self, task: TTask) -> TTaskAdder;

    fn add_task_box(self, task_box: TaskBox) -> TTaskAdder;

    fn add_task_boxes<TTaskBoxIntoIterator: 'static + TaskBoxIntoIterator>(self, task_boxes: TTaskBoxIntoIterator) -> TTaskAdder;
}
