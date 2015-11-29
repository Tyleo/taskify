use ContinuationAdder;
use ContinuationAdderTrait;
use EndScheduleTrait;
use LooseContinuation;
use LooseContinuationIntoIterator;
use Task;
use TaskAdderTrait;
use TaskBox;
use TaskBoxIntoIterator;

pub struct TaskAdder;

impl TaskAdder {
    fn ConvertToContinuationAdder(self) -> ContinuationAdder {
        ContinuationAdder
    }
}

impl TaskAdderTrait<ContinuationAdder, TaskAdder> for TaskAdder {
    fn add_task<TTask: 'static + Task>(self, task: TTask) -> TaskAdder {
        self
    }

    fn add_task_box(self, task_box: TaskBox) -> TaskAdder {
        self
    }

    fn add_task_boxes<TTaskBoxIntoIterator: 'static + TaskBoxIntoIterator>(self, task_boxes: TTaskBoxIntoIterator) -> TaskAdder {
        self
    }
}

impl ContinuationAdderTrait<ContinuationAdder> for TaskAdder {
    fn add_continuation<TTask: 'static + Task>(self, continuation: TTask) -> ContinuationAdder {
        self.ConvertToContinuationAdder()
            .add_continuation(continuation)
    }

    fn add_continuation_box(self, continuation_box: TaskBox) -> ContinuationAdder {
        self.ConvertToContinuationAdder()
            .add_continuation_box(continuation_box)
    }

    fn add_continuation_boxes<TTaskBoxIntoIterator: 'static + TaskBoxIntoIterator>(self, continuation_boxes: TTaskBoxIntoIterator) -> ContinuationAdder {
        self.ConvertToContinuationAdder()
            .add_continuation_boxes(continuation_boxes)
    }

    fn add_loose_continuation(self, loose_continuation: LooseContinuation) -> ContinuationAdder {
        self.ConvertToContinuationAdder()
            .add_loose_continuation(loose_continuation)
    }

    fn add_loose_continuations<TLooseContinuationIntoIterator: 'static + LooseContinuationIntoIterator>(self, loose_continuations: TLooseContinuationIntoIterator) -> ContinuationAdder {
        self.ConvertToContinuationAdder()
            .add_loose_continuations(loose_continuations)
    }
}

impl EndScheduleTrait for TaskAdder {
    fn EndSchedule(self) {
        self.ConvertToContinuationAdder()
            .EndSchedule();
    }
}
